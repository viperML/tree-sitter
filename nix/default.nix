{pkgs ? import <nixpkgs> {}}: let
  inherit (pkgs) lib;
  inherit (lib) filterAttrs;
  inherit (builtins) mapAttrs removeAttrs attrValues;

  nv = pkgs.callPackages ./generated.nix {};

  nvGrammars = pkgs.callPackages ./grammars/generated.nix {};
  fullGrammarName = name: "tree-sitter-${name}";
  filterGrammars = g: l: removeAttrs g (map fullGrammarName l);

  cargoToml = builtins.fromTOML (builtins.readFile ../Cargo.toml);
in
  lib.fix (self:
    {
      tree-sitter = pkgs.callPackage ./tree-sitter.nix {};

      nvim-treesitter = nv.nvim-treesitter.src.overrideAttrs (old: rec {
        pname = "nvim-treesitter";
        version = nv.nvim-treesitter.date;
        name = "${pname}-${version}";
      });

      neovim = pkgs.callPackage ./nvim.nix {
        inherit (self) nvim-treesitter;
      };

      grammars = {
        all = mapAttrs (name: value:
          pkgs.callPackage ./grammars/grammar.nix {
            nv = value;
            inherit (self) tree-sitter;
          })
        nvGrammars;

        filtered = filterGrammars self.grammars.all [
          "csv"
          "cuda"
          "ebnf"
          "elvish"
          "gdscript"
          "godot_resource"
          "gowork"
          "graphql"
          "haskell_persistent"
          "hjson"
          "htmldjango"
          "jq"
          "json5"
          "jsonc"
          "kusto"
          "liquid"
          "markdown"
          "markdown_inline"
          "menhir"
          "nim_format_string"
          "norg"
          "org"
          "passwd"
          "perl"
          "pioasm"
          "pod"
          "prisma"
          "promql"
          "prql"
          "psv"
          "rnoweb"
          "sql"
          "styled"
          "surface"
          "tsv"
          "v"
          "yang"
        ];
      };

      nvim-grammars = {
        all = mapAttrs (name: nv:
          pkgs.callPackage ./grammars/nvim-grammar.nix {
            inherit nv;
          })
        nvGrammars;

        filtered = filterGrammars self.nvim-grammars.all [];
      };

      grammar-bundle = pkgs.linkFarmFromDrvs "tree-sitter-grammar-bundle" (attrValues self.grammars.filtered);
      nvim-grammar-bundle = pkgs.linkFarmFromDrvs "tree-sitter-nvim-grammar-bundle" (attrValues self.nvim-grammars.filtered);
    }
    // (lib.genAttrs cargoToml.workspace.members (member:
      pkgs.callPackage ./packages.nix {
        inherit member;
        tsGrammarPath = "${self.grammar-bundle}";
      })))
