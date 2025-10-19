{
  pkgs,
  config,
  inputs,
  ...
}: let
  oic-fox-fuckery = config.languages.rust.import ./. {};
  project_name = "oic-fox-fuckery";
  registry_user = "doot";
  tag = "latest";
  oic-fox-fuckery-prod-deriv = inputs.nix2container.packages.x86_64-linux.nix2container.buildImage {
    # Use nix2container directly since devenv containers include the entire environment, which is several GBs. This way the container is < 100 MB
    name = "${registry_user}/${project_name}";
    inherit tag;
    config = {
      Cmd = ["start" "--environment" "production" "--binding" "0.0.0.0"];
      ExposedPorts = {
        "5150/tcp" = {};
      };
      EntryPoint = ["bin/oic-fox-fuckery-cli"];
    };
    copyToRoot = pkgs.buildEnv {
      name = "image-root";
      paths = [oic-fox-fuckery ./. ./config];
      pathsToLink = ["/bin" "/config"];
    };
  };
in {
  name = project_name;

  cachix.pull = ["pre-commit-hooks"];

  languages = {
    nix.enable = !config.container.isBuilding;
    rust = {
      enable = true;
      channel = "stable";
      mold.enable = true;
    };
  };
  # TODO: there is an unhandled error when the api key is bad...

  outputs = {
    inherit oic-fox-fuckery;
    oic-fox-fuckery-prod-local = oic-fox-fuckery-prod-deriv.copyToDockerDaemon;
    oic-fox-fuckery-prod-push = oic-fox-fuckery-prod-deriv.copyToRegistry;
  };

  tasks = {
    "container:local" = {
      exec = ''
        set -euo pipefail

        echo "Building docker image and copying it to local docker daemon..."

        copyscript=$(devenv build outputs.oic-fox-fuckery-prod-local)

        echo "Loading image into docker daemon via $copyscript..."
        $copyscript/bin/copy-to-docker-daemon

        echo "Loaded container into local docker daemon: ${registry_user}/${project_name}":${tag}
        echo '{ "image": "${registry_user}/${project_name}:${tag}" }' > $DEVENV_TASK_OUTPUT_FILE
      '';
      execIfModified = [
        "src/**/*.rs"
        "config/**/*.yaml"
        "*.toml"
        "devenv.nix"
      ];
    };

    "container:registry" = {
      exec = ''
        set -euo pipefail

        echo "Building docker image and copying it to remote registry..."

        copyscript=$(devenv build outputs.oic-fox-fuckery-prod-push)

        echo "Loading image into docker daemon via $copyscript..."
        $copyscript/bin/copy-to-registry --dest-creds ${registry_user}:$REGISTRY_API_KEY

        echo "Copied container into remote registry: ${registry_user}/${project_name}:${tag}"
        echo '{ "image": "${registry_user}/${project_name}:${tag}" }' > $DEVENV_TASK_OUTPUT_FILE
      '';
      execIfModified = [
        "src/**/*.rs"
        "config/**/*.yaml"
        "*.toml"
        "devenv.nix"
      ];
    };
  };

  packages =
    if config.container.isBuilding
    then [oic-fox-fuckery]
    else [
      pkgs.git
      pkgs.bacon
      pkgs.atop
      pkgs.loco
      oic-fox-fuckery
    ];

  # https://devenv.sh/processes/
  # processes = {
  #   watch.exec = "bacon run";
  #   clippy.exec = "bacon clippy-all";
  # };

  containers = {
    # TODO: This container still includes the entire dev environment, making it 3-4 GB. It should not be used until there is a way to only include the rust
    # binary. Use the nix2container output above instead.
    prod = {
      name = "oic-fox-fuckery";
      entrypoint = ["bin/oic-fox-fuckery-cli"];
      startupCommand = "start";
      copyToRoot = pkgs.buildEnv {
        name = "image-root";
        paths = [
          oic-fox-fuckery
          ./config
          ./.
          # TODO: This puts every file in config dir into the root and there does not appear to be a way to get the config dir itself
        ];
        pathsToLink = ["/bin" "/config"];
      };
    };
  };

  delta.enable = true;

  # https://devenv.sh/tests/
  enterTest = ''
    echo "Running tests"
    cargo fmt --check
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
      # commitizen.enable = true;  # TODO: Temporarily disable until fix makes it downstream
      deadnix.enable = true;
      statix.enable = true;
      alejandra.enable = true;
      markdownlint = {
        enable = true;
        settings.configuration = {
          MD013 = {
            line_length = 180;
          };
        };
      };
      check-json.enable = true;
      pretty-format-json = {
        enable = true;
        args = ["--no-sort-keys"];
      };
      cargo-check.enable = true;
      clippy = {
        enable = true;
        settings.allFeatures = true;
        settings.denyWarnings = true;
      };
      rustfmt = {
        enable = true;
        settings.config-path = ".rustfmt.toml";
      };
      check-toml.enable = true;
      check-yaml.enable = true;
    };
  };

  devcontainer.enable = true;
}
