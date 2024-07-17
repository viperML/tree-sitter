with import <nixpkgs> {};
  linkFarm "tsg" [
    {
      name = "tree-sitter-html";
      path = tree-sitter-grammars.tree-sitter-html;
    }
  ]
