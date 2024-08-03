{
  neovim,
  nvim-treesitter,
}:
neovim.override {
  extraLuaPackages = lp: [
  ];
  configure = {
    customRC =
      # lua
      ''
        lua << EOF
        require("nvim-treesitter").setup {}
        EOF
      '';
    packages.main = {
      start = [
        nvim-treesitter
      ];
    };
  };
}
