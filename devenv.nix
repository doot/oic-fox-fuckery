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
      Env = ["SSL_CERT_FILE=/etc/ssl/certs/ca-bundle.crt"];
      ExposedPorts = {
        "5150/tcp" = {};
      };
      EntryPoint = ["bin/${project_name}-cli"];
    };
    copyToRoot = pkgs.buildEnv {
      name = "image-root";
      paths = [oic_fox_fuckery_cli pkgs.cacert ./. ./config];
      pathsToLink = ["/bin" "/config" "/etc/ssl/certs"];
    };
  };
in {
  name = lib.mkForce project_name;

  outputs = {
    inherit oic_fox_fuckery_cli;
    prod_image_copy_local = prod_image.copyToDockerDaemon;
    prod_image_copy_registry = prod_image.copyToRegistry;
  };

  packages = lib.optionals config.container.isBuilding [
    oic_fox_fuckery_cli # Project package
  ];

  # packages =
  #   lib.optionals (!config.container.isBuilding && !config.devenv.isTesting) [
  #     # Development packages to include only when not building a container or testing
  #     pkgs.bacon
  #     pkgs.atop
  #     pkgs.loco
  #     pkgs.statix
  #     pkgs.deadnix
  #     pkgs.nil
  #     pkgs.jq # Needed for tasks and CLI script that use jq
  #     pkgs.lldb
  #   ]
  #   ++ lib.optionals config.container.isBuilding [
  #     oic_fox_fuckery_cli # Project package
  #   ];

  tasks = {
    "container:local" = {
      exec = ''
        set -euo pipefail

        echo "Building docker image and copying it to local docker daemon..."

        copyscript=$(devenv build -q outputs.prod_image_copy_local | jq -r '.["outputs.prod_image_copy_local"]')

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

        copyscript=$(devenv build -q outputs.prod_image_copy_registry | jq -r '.["outputs.prod_image_copy_registry"]')

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
    "$(devenv build -q outputs.oic_fox_fuckery_cli | jq -r '.["outputs.oic_fox_fuckery_cli"]')/bin/${project_name}-cli" "$@"
  '';
}
