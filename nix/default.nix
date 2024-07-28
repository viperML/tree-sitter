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

  all-grammars = pkgs.linkFarm "tree-sitter-all-grammars" (lib.pipe grammars [
    (lib.flip builtins.removeAttrs (lib.pipe [
        "perl"
        "pod"
        "sql"
      ] [
        (map (lang: "tree-sitter-${lang}"))
        (map (p: lib.warn "all-grammars: skipping ${p} due to build failures" p))
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
