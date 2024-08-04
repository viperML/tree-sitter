{pkgs ? import <nixpkgs> {}}: let
  inherit (builtins) mapAttrs;

  grammar-srcs = pkgs.callPackages ./grammars/generated.nix {};

  nvim-treesitter = let
    nv = (pkgs.callPackages ./generated.nix {}).nvim-treesitter;
  in
  nv.src.overrideAttrs (old: rec {
      pname = "nvim-treesitter";
      version = nv.date;
      name = "${pname}-${version}";
    });

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
