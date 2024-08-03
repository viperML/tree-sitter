with import <nixpkgs> {};
  tree-sitter-grammars.tree-sitter-typescript.overrideAttrs (old: {
    postInstall =
      # bash
      ''
        pwd
        ls -la
      '';
  })
