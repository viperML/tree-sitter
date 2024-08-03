{pkgs ? import <nixpkgs> {}}: let
  inherit (builtins) mapAttrs;
  inherit (pkgs) lib;

  grammar-srcs = pkgs.callPackages ./grammars/generated.nix {};

  meta = builtins.fromJSON (builtins.readFile ./grammars/meta.json);

  nvim-treesitter = (pkgs.callPackages ./generated.nix {}).nvim-treesitter.src;

  tree-sitter = pkgs.callPackage ./tree-sitter.nix {};

  grammars = mapAttrs (name: value:
    pkgs.callPackage ./grammars/grammar.nix {
      nv = value;
      meta = meta.${lib.removePrefix "tree-sitter-" name};
      inherit nvim-treesitter tree-sitter;
    })
  grammar-srcs;
in {
  inherit grammars tree-sitter nvim-treesitter;

  nvim = pkgs.callPackage ./nvim.nix {
    inherit nvim-treesitter;
  };
}
