{
  inputs.nixpkgs.url = "github:NixOS/nixpkgs?ref=nixos-unstable";

  outputs = {
    self,
    nixpkgs,
  }: let
    system = "x86_64-linux";
    pkgs = nixpkgs.legacyPackages.${system};
    inherit (pkgs) lib;

    default = import ./nix {inherit pkgs;};
    cargoToml = builtins.fromTOML (builtins.readFile ./Cargo.toml);
  in {
    packages.${system} =
      (lib.genAttrs cargoToml.workspace.members (member:
        pkgs.callPackage ./nix/packages.nix {
          inherit member;
          ts-grammar-path = self.packages.${system}.bundle;
        }))
      // {
        inherit (default) tree-sitter nvim-treesitter;

        bundle = pkgs.linkFarmFromDrvs "tree-sitter-bundle" (lib.flatten [
          (builtins.attrValues self.legacyPackages.${system}.grammars.filtered)
          default.nvim-treesitter
        ]);

        bundle-dev = pkgs.linkFarmFromDrvs "tree-sitter-bundle" (builtins.attrValues self.legacyPackages.${system}.grammars.dev);
      };

    legacyPackages.${system} = {
      grammars = {
        all = default.grammars;
        filtered = builtins.removeAttrs default.grammars [
          "tree-sitter-norg"
          # "tree-sitter-apex"
          "tree-sitter-cuda"
          "tree-sitter-csv"
          "tree-sitter-psv"
          "tree-sitter-tsv"
          "tree-sitter-ebnf" # in subdir
          "tree-sitter-markdown"
          "tree-sitter-markdown_inline"
          "tree-sitter-perl" # wtf
          "tree-sitter-pod" # same repo
          "tree-sitter-promql"
          "tree-sitter-sxhkdrc"
          "tree-sitter-styled"
          "tree-sitter-v"
          "tree-sitter-sql"
        ];
        # Selection of grammars with quirks
        dev =
          lib.getAttrs (map (n: "tree-sitter-${n}") [
            "javascript"
            "nix"
            "latex"
            "php"
            "php_only"
            "typescript"
            "tsx"
            "csv"
          ])
          default.grammars;
      };
    };

    devShells.${system}.default = pkgs.mkShell {
      packages = [
        pkgs.cargo
        pkgs.rustc
        pkgs.rust-analyzer
        pkgs.rustfmt
        pkgs.clippy
        pkgs.gdb
        pkgs.pkg-config
        pkgs.file

        self.packages.${system}.tree-sitter
        (pkgs.python3.withPackages (p: [
          p.python-lsp-server
        ]))
        pkgs.ruff
      ];
      env.RUST_SRC_PATH = "${pkgs.rustPlatform.rustLibSrc}";
    };
  };
}
