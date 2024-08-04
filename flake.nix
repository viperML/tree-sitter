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
          ts-grammar-path = self.packages.${system}.bundle;
        }))
      // {
        inherit (default) tree-sitter nvim-treesitter;

        bundle = pkgs.linkFarm "tree-sitter-bundle" ((lib.pipe self.legacyPackages.${system}.grammars.filtered [
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
        filtered = builtins.removeAttrs default.grammars ["tree-sitter-norg"];
      };
    };
  };
}
