with import <nixpkgs> {};
  neovim.override {
    configure = {
      customRC =
        # lua
        ''
          lua << EOF
          require("nvim-treesitter").setup {}
          EOF
        '';
      packages.main = {
        start = with vimPlugins; [
          nvim-treesitter
        ];
      };
    };
  }
