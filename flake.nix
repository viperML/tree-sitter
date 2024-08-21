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
        filtered = builtins.removeAttrs default.grammars (map (n: "tree-sitter-${n}") [
          "norg"
          # "apex"
          "cuda"
          "csv"
          "psv"
          "tsv"
          "elvish"
          "gowork"
          "gomod"
          "ebnf" # in subdir
          "markdown"
          "markdown_inline"
          "perl" # wtf
          "pod" # same repo
          "promql"
          "sxhkdrc"
          "styled"
          "v"
          "sql"
          "graphql"
          "hjson"
          "glimmer"
          "regex"
          "pioasm"
          "prisma"
          "surface"
          "gdscript"
          "yang"
          "json5"
          "org"
          "groovy"
          "godot_resource"
          "haskell_persistent"
          "jq"
          "prql"
          "menhir"
          "liquid"
          "htmldjango"
          "rnoweb"
          "nim_format_string"
          "kusto"
          "jsonc"
          "passwd"
        ]);
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
            "rust"
            # "csv"
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
        pkgs.nodejs

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
