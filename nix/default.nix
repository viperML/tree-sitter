{pkgs ? import <nixpkgs> {}}: let
  inherit (builtins) mapAttrs;

  grammar-srcs = pkgs.callPackages ./grammars/generated.nix {};

  nvim-treesitter = (pkgs.callPackages ./generated.nix {}).nvim-treesitter.src;

  tree-sitter = pkgs.callPackage ./tree-sitter.nix {};

  grammars = mapAttrs (name: value:
    pkgs.callPackage ./grammars/grammar.nix {
      nv = value;
      inherit tree-sitter;
    })
  grammar-srcs;
in {
  inherit grammars tree-sitter nvim-treesitter;

  nvim = pkgs.callPackage ./nvim.nix {
    inherit nvim-treesitter;
  };
}
