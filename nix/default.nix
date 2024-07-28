{pkgs ? import <nixpkgs> {}}: let
  inherit (builtins) mapAttrs;
  inherit (pkgs) lib;

  sources = pkgs.callPackages ./generated.nix {};
  meta = builtins.fromTOML (builtins.readFile ./meta.toml);

  grammars = mapAttrs (name: value:
    pkgs.callPackage ./grammar.nix {
      nv = value;
      meta = meta.${name};
    })
  sources;

  filteredGrammars = builtins.removeAttrs grammars (lib.pipe [
      "perl"
      "pod"
      "sql"
    ] [
      (map (lang: "tree-sitter-${lang}"))
      (map (p: lib.warn "all-grammars: skipping ${p} due to build failures" p))
    ]);
in {
  inherit grammars filteredGrammars;

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
}
