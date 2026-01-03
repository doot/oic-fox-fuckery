{
  pkgs,
  config,
  inputs,
  lib,
  ...
}: let
  project_name = "oic-fox-fuckery";
  oic_fox_fuckery_cli = config.languages.rust.import ./. {};
  registry_user = "doot";
  tag = "latest";
  prod_image = inputs.nix2container.packages.x86_64-linux.nix2container.buildImage {
    # Use nix2container directly since devenv containers include the entire environment, which is several GBs. This way the container is < 100 MB
    name = "${registry_user}/${project_name}";
    inherit tag;
    config = {
      Cmd = ["start" "--environment" "production" "--binding" "0.0.0.0"];
      ExposedPorts = {
        "5150/tcp" = {};
      };
      EntryPoint = ["bin/${project_name}-cli"];
    };
    copyToRoot = pkgs.buildEnv {
      name = "image-root";
      paths = [oic_fox_fuckery_cli ./. ./config];
      pathsToLink = ["/bin" "/config"];
    };
  };
in {
  name = project_name;

  outputs = {
    inherit oic_fox_fuckery_cli;
    prod_image_copy_local = prod_image.copyToDockerDaemon;
    prod_image_copy_registry = prod_image.copyToRegistry;
  };

  cachix.pull = ["pre-commit-hooks"];

  devcontainer.enable = true;

  delta.enable = true;

  languages = {
    nix.enable = !config.container.isBuilding && !config.devenv.isTesting;
    rust = {
      enable = true;
      toolchainFile = ./rust-toolchain.toml;
      rustflags = "-Z threads=8";
      mold.enable = true;
    };
  };

  packages =
    lib.optionals (!config.container.isBuilding && !config.devenv.isTesting) [
      # Development packages to include only when not building a container or testing
      pkgs.bacon
      pkgs.atop
      pkgs.loco
      pkgs.statix
      pkgs.deadnix
      pkgs.nil
    ]
    ++ lib.optionals config.container.isBuilding [
      oic_fox_fuckery_cli # Project package
    ];

  tasks = {
    "container:local" = {
      exec = ''
        set -euo pipefail

        echo "Building docker image and copying it to local docker daemon..."

        copyscript=$(devenv build outputs.prod_image_copy_local)

        echo "Loading image into docker daemon via $copyscript..."
        $copyscript/bin/copy-to-docker-daemon

        echo "Loaded container into local docker daemon: ${registry_user}/${project_name}:${tag}"
        echo '{ "image": "${registry_user}/${project_name}:${tag}" }' > $DEVENV_TASK_OUTPUT_FILE
      '';
      execIfModified = [
        "src/**/*.rs"
        "config/**/*.yaml"
        "*.toml"
        "devenv.nix"
        "*.lock"
      ];
    };

    "container:registry" = {
      exec = ''
        set -euo pipefail

        echo "Building docker image and copying it to remote registry..."

        copyscript=$(devenv build outputs.prod_image_copy_registry)

        echo "Pushing image to registry via $copyscript..."
        $copyscript/bin/copy-to-registry --dest-creds ${registry_user}:$REGISTRY_API_KEY

        echo "Pushed container into remote registry: ${registry_user}/${project_name}:${tag}"
        echo '{ "image": "${registry_user}/${project_name}:${tag}" }' > $DEVENV_TASK_OUTPUT_FILE
      '';
      execIfModified = [
        "src/**/*.rs"
        "*.toml"
        "config/**/*.yaml"
        "devenv.nix"
        "*.lock"
      ];
    };
  };

  containers = {
    # TODO: This container still includes the entire dev environment, making it 3-4 GB. It should not be used until there is a way to only include the rust
    # binary. Use the nix2container output above instead.
    prod = {
      name = project_name;
      entrypoint = ["bin/${project_name}-cli"];
      startupCommand = "start";
      copyToRoot = pkgs.buildEnv {
        name = "image-root";
        paths = [
          oic_fox_fuckery_cli
          ./config
          ./.
          # TODO: This puts every file in config dir into the root and there does not appear to be a way to get the config dir itself
        ];
        pathsToLink = ["/bin" "/config"];
      };
    };
  };

  # Hacky way to avoid putting CLI in 'packages' and delay building the CLI until needed
  scripts."${project_name}-cli".exec = ''
    $(devenv build -q outputs.oic_fox_fuckery_cli)/bin/${project_name}-cli "$@"
  '';

  enterTest = ''
    echo "Running tests"
    cargo fmt --check
    cargo build
    cargo clippy --all-targets --all-features
    cargo test
  '';

  enterShell = ''
    echo "Rust version: $(rustc --version)"
    echo "Cargo version: $(cargo --version)"
    echo "RUST_SRC_PATH: $RUST_SRC_PATH"
  '';

  git-hooks = {
    hooks = {
      commitizen.enable = !config.container.isBuilding;
      deadnix.enable = !config.container.isBuilding;
      statix.enable = !config.container.isBuilding;
      alejandra.enable = !config.container.isBuilding;
      markdownlint = {
        enable = !config.container.isBuilding;
        settings.configuration = {
          MD013 = {
            line_length = 180;
          };
        };
      };
      check-json.enable = !config.container.isBuilding;
      pretty-format-json = {
        enable = !config.container.isBuilding;
        args = ["--no-sort-keys"];
      };
      cargo-check.enable = !config.container.isBuilding;
      clippy = {
        enable = true;
        settings.allFeatures = true;
        settings.denyWarnings = true;
      };
      rustfmt = {
        enable = !config.container.isBuilding;
        settings.config-path = ".rustfmt.toml";
      };
      check-toml.enable = !config.container.isBuilding;
      check-yaml.enable = !config.container.isBuilding;
    };
  };
}
