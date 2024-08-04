{
  inputs.nixpkgs.url = "github:NixOS/nixpkgs?ref=nixos-unstable";

  outputs = {
    self,
    nixpkgs,
  }: let
    system = "x86_64-linux";
    pkgs = nixpkgs.legacyPackages.${system};
    inherit (pkgs) lib;

    default = import ./nix {inherit pkgs;};
    cargoToml = builtins.fromTOML (builtins.readFile ./Cargo.toml);
  in {
    packages.${system} =
      (lib.genAttrs cargoToml.workspace.members (member:
        pkgs.callPackage ./nix/packages.nix {
          inherit member;
        }))
      // {
        inherit (default) tree-sitter nvim-treesitter;

        all-grammars = pkgs.linkFarm "all-grammars" ((lib.pipe default.grammars [
            (lib.flip builtins.removeAttrs [
              "tree-sitter-norg"
            ])
            lib.attrsToList
            (map ({
              name,
              value,
            }: {
              inherit name;
              path = value;
            }))
          ])
          ++ [
            {
              name = "nvim-treesitter";
              path = default.nvim-treesitter;
            }
          ]);
      };

    legacyPackages.${system} = {
      grammars = {
        all = default.grammars;
        # filtered = lib.filterAttrs (name: value: true) default.grammars;
      };
    };
  };
}
