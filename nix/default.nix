{pkgs ? import <nixpkgs> {}}: let
  inherit (builtins) mapAttrs;
  inherit (pkgs) lib;

  sources = pkgs.callPackages ./generated.nix {};
  meta = builtins.fromJSON (builtins.readFile ./meta.json);

  nvim-treesitter-base = (pkgs.callPackages ./ts/generated.nix {}).nvim-treesitter.src;

  tree-sitter = pkgs.callPackage ./tree-sitter.nix {};

  grammars =
    (mapAttrs (name: value:
      pkgs.callPackage ./grammar.nix {
        nv = value;
        meta = meta.${lib.removePrefix "tree-sitter-" name};
        nvim-treesitter = nvim-treesitter-base;
        inherit tree-sitter;
      })
    sources)
    // {
      nvim-treesitter = nvim-treesitter-base;
    };

  filteredGrammars = builtins.removeAttrs grammars (lib.pipe [
      "perl"
      "pod"
      "sql"
      # "csv"
    ] [
      (map (lang: "tree-sitter-${lang}"))
      (map (p: lib.warn "all-grammars: skipping ${p} due to build failures" p))
    ]);

  selectedGrammars = lib.getAttrs ((map (g: "tree-sitter-${g}") [
      "rust"
      "latex"
      "javascript"
      "typescript"
      "astro"
      "html"
      "scss"
    ])
    ++ ["nvim-treesitter"])
  grammars;
in {
  inherit grammars filteredGrammars nvim-treesitter-base tree-sitter;

  nvim = pkgs.callPackage ./nvim.nix {
    nvim-treesitter = nvim-treesitter-base;
  };

  tree-sitter-all = pkgs.linkFarm "tree-sitter-all" (lib.pipe grammars [
    lib.attrsToList
    (map ({
      name,
      value,
    }: {
      inherit name;
      path = value;
    }))
  ]);

  tree-sitter-all-filtered = pkgs.linkFarm "tree-sitter-all" (lib.pipe filteredGrammars [
    lib.attrsToList
    (map ({
      name,
      value,
    }: {
      inherit name;
      path = value;
    }))
  ]);

  tree-sitter-selected = pkgs.linkFarm "tree-sitter-selected" (lib.pipe selectedGrammars [
    lib.attrsToList
    (map ({
      name,
      value,
    }: {
      inherit name;
      path = value;
    }))
  ]);
}
