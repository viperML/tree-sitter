with import <nixpkgs> {};
  linkFarm "tsg" (lib.pipe tree-sitter-grammars [
    lib.attrsToList
    (builtins.filter ({name, value}: lib.isDerivation value))
    (map ({name, value}: {
      inherit name;
      path = value;
    }))
  ])
