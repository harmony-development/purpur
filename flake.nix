{
  inputs.nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
  inputs.nci.url = "github:yusdacra/nix-cargo-integration";
  inputs.nci.inputs.nixpkgs.follows = "nixpkgs";
  inputs.parts.url = "github:hercules-ci/flake-parts";
  inputs.parts.inputs.nixpkgs-lib.follows = "nixpkgs";

  outputs = inputs @ {
    parts,
    nci,
    ...
  }:
    parts.lib.mkFlake {inherit inputs;} {
      systems = ["x86_64-linux"];
      imports = [
        nci.flakeModule
      ];
      perSystem = {
        pkgs,
        config,
        ...
      }: let
        crateOutputs = config.nci.outputs;
      in {
        nci.toolchainConfig = {
          channel = "stable";
          components = ["rust-analyzer" "rust-src" "clippy" "rustfmt"];
        };
        nci.projects."purpur" = {
          path = ./.;
          export = true;
        };
        nci.crates = {
          "libpurpur" = {};
          "app" = {};
          "libpurpurc" = {};
        };
        devShells.default = crateOutputs.purpur.devShell.overrideAttrs (old: {
          packages = (old.packages or []) ++ (with pkgs; [pkg-config gtk4 openssl libsodium.dev sqlite]);
        });
        packages.default = crateOutputs."app".packages.release;
      };
    };
}
