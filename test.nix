with import <nixpkgs> {};
tree-sitter-grammars.tree-sitter-typescript.overrideAttrs (old: {
  postInstall = ''
    pwd
    ls -la
  '';
})
