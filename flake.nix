{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs?ref=nixos-unstable";
    flake-parts = {
      url = "github:hercules-ci/flake-parts";
      inputs.nixpkgs-lib.follows = "nixpkgs";
    };
  };

  outputs = inputs @ {
    nixpkgs,
    flake-parts,
    ...
  }:
    flake-parts.lib.mkFlake {inherit inputs;} ({lib, ...}: {
      systems = [
        "x86_64-linux"
      ];

      perSystem = {
        pkgs,
        config,
        ...
      }: let
        result = import ./nix {inherit pkgs;};
      in {
        packages = lib.filterAttrs (_: lib.isDerivation) result;

        legacyPackages = {
          inherit (result) grammars nvim-grammars;
        };

        devShells.default = with pkgs;
          mkShell {
            packages = [
              cargo
              rustc
              rust-analyzer
              rustfmt
              clippy
              pkg-config
              file
              nodejs
              config.packages.tree-sitter
              (python3.withPackages (pp: [
                pp.python-lsp-server
              ]))
              ruff
            ];
            env.RUST_SRC_PATH = "${rustPlatform.rustLibSrc}";
          };
      };
    });
}
