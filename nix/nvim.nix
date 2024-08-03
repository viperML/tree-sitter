{
  neovim,
  nvim-treesitter,
  vimUtils,
}:
neovim.override {
  extraLuaPackages = lp: [
    # lp.rapidjson
    # lp.lua-rtoml
    # lp.lua-cjson
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
        # (vimUtils.buildVimPlugin {
        #   name = "nvim-treesitter";
        #   src = nvim-treesitter;
        # })
        nvim-treesitter
      ];
    };
  };
}
