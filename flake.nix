{
  description = "Orcfax consensus in rust";

  inputs = {
    flake-parts.url = "github:hercules-ci/flake-parts";
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    git-hooks-nix.url = "github:cachix/git-hooks.nix";
    git-hooks-nix.inputs.nixpkgs.follows = "nixpkgs";
    treefmt-nix.url = "github:numtide/treefmt-nix";
    treefmt-nix.inputs.nixpkgs.follows = "nixpkgs";
    rust-flake.url = "github:juspay/rust-flake/";
  };

  outputs = inputs @ {flake-parts, ...}:
    flake-parts.lib.mkFlake {inherit inputs;}
    {
      imports = [
        inputs.git-hooks-nix.flakeModule
        inputs.treefmt-nix.flakeModule
        inputs.rust-flake.flakeModules.default
        inputs.rust-flake.flakeModules.nixpkgs
      ];
      systems = ["x86_64-linux" "aarch64-darwin"];
      perSystem = {
        lib,
        config,
        pkgs,
        ...
      }: let
        clang-unwrapped = pkgs.llvmPackages_latest.clang-unwrapped;
        devShell = {
          name = "dev-shell";
          shellHook = ''
            ${config.pre-commit.installationScript}
            echo 1>&2 "Welcome to the development shell!"
            export RUST_SRC_PATH="${config.rust-project.toolchain}/lib/rustlib/src/rust/library";
          '';
          packages =
            [
              # pkgs.openssl
              config.rust-project.toolchain
              # clang-unwrapped
              # DOCUMENTATION
              pkgs.mermaid-cli
              pkgs.pandoc
              pkgs.just
              pkgs.svgo
            ]
            ++ lib.mapAttrsToList (_: crate: crate.crane.args.nativeBuildInputs) config.rust-project.crates;
          buildInputs =
            [
              pkgs.libiconv
            ]
            ++ lib.mapAttrsToList (_: crate: crate.crane.args.buildInputs) config.rust-project.crates;
          nativeBuildInputs = [
            config.treefmt.build.wrapper
          ];
          # Not using wasm in this project
          # CC_wasm32_unknown_unknown = lib.getExe' clang-unwrapped "clang";
        };
      in {
        rust-project = {
          toolchain = pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;
        };
        treefmt = {
          projectRootFile = "flake.nix";
          flakeFormatter = true;
          programs = {
            prettier = {
              enable = true;
              settings = {
                printWidth = 80;
                proseWrap = "always";
              };
            };
            alejandra.enable = true;
            rustfmt.enable = true;
            taplo.enable = true;
          };
        };
        pre-commit.settings.hooks = {
          treefmt.enable = true;
        };
        devShells = {
          default = pkgs.mkShell devShell;
        };
      };
    };
}
