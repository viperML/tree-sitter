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
in {
  inherit grammars;

  allGrammars = pkgs.linkFarm "tree-sitter-all-grammars" (lib.pipe grammars [
    (lib.flip builtins.removeAttrs (map (lang: "tree-sitter-${lang}") [
      "perl"
      "pod"
      "sql"
    ]))
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
