# viperML/tree-sitter

I created this repository to fill some holes in the tree-sitter ecosystem.


## nix

This flake provides all compiled tree-sitter grammars.

Two interfaces are provided:

```
packages.<system>.nvim-treesitter

:: AttrSet
legacyPackages.<system>.grammars.filtered

:: AttrSet
legacyPackages.<system>.nvim-grammars.filtered
```

- `nvim-treesitter` is the Neovim plugin. Use this instead of
`vimPlugins.nvim-treesitter`, as the versions of the grammars are compatible
with this plugin version.
- `nvim-grammars` are meant to be used with neovim. You can concat 
`builtins.attrValues nvim-grammars.filtered` with your plugin list, or filter 
them by name.
- `grammars` are meant to be used with the rest the tree-sitter, like the 
upstream CLI tool or my crates.
