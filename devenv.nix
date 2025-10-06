{
  pkgs,
  config,
  ...
}: let
  oic-fox-fuckery = config.languages.rust.import ./. {};
in {
  name = "oic-fox-fuckery";

  languages = {
    nix.enable = !config.containers.prod.isBuilding;
    rust = {
      enable = !config.containers.prod.isBuilding;
      channel = "stable";
      mold.enable = true;
    };
  };

  outputs = {
    inherit oic-fox-fuckery;
  };

  packages = with pkgs;
    if config.containers.prod.isBuilding
    then [oic-fox-fuckery]
    else [
      git
      bacon
      atop
      loco
    ];

  # https://devenv.sh/processes/
  # processes = {
  #   watch.exec = "bacon run";
  #   clippy.exec = "bacon clippy-all";
  # };

  containers = {
    prod = {
      name = "oic-fox-fuckery";
      # startupCommand = config.processes.serve.exec;
      startupCommand = "${oic-fox-fuckery}/bin/oic-fox-fuckery-cli start";
      copyToRoot = [
        oic-fox-fuckery
        # This works? but contains everything.
        ./.
        # TODO: This puts every file in config dir into the root and there does not appear to be a way to get the config dir itself
        #  ./config
        #  One possibility is to put it one level deeper...
      ];
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
