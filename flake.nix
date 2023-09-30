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
        ./crates.nix
      ];
      perSystem = {
        pkgs,
        config,
        ...
      }: let
        crateOutputs = config.nci.outputs."purpur";
      in {
        nci.toolchainConfig = {
          channel = "stable";
          components = ["rust-analyzer" "rust-src" "clippy" "rustfmt"];
        };
        devShells.default = crateOutputs.devShell.overrideAttrs (old: {
          packages = (old.packages or []) ++ (with pkgs; [ pkg-config gtk4 openssl libsodium.dev sqlite ]);
        });
        packages.default = crateOutputs.packages.release;
      };
    };
}
