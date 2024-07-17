with import <nixpkgs> {};
  linkFarm "tsg" (lib.pipe tree-sitter-grammars [
    lib.attrValues
    (builtins.filter lib.isDerivation)
    (map (grammar: {
      name = grammar.pname;
      path = grammar;
    }))
  ])
