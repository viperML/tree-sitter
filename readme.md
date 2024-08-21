# viperML/tree-sitter

I created this repository to fill some holes in the tree-sitter ecosystem.


## nix

This flake provides all compiled tree-sitter grammars.

Two interfaces are provided:

```
:: AttrSet
legacyPackages.<system>.grammars.filtered

:: AttrSet
legacyPackages.<system>.nvim-grammars.filtered
```

- `nvim-grammars` are meant to be used with neovim. You can concat `builtins.attrValues nvim-grammars.filtered` with your plugin list, or filter them by name.
- `grammars` are meant to be used with the rest the tree-sitter, like the upstream CLI tool or my crates.
