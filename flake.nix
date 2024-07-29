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
    packages.${system} = lib.genAttrs cargoToml.workspace.members (member:
      pkgs.callPackage ./nix/packages.nix {
        inherit member;
      });

    legacyPackages.${system} = {
      grammars = {
        all = default.grammars;
        filtered = default.filteredGrammars;
      };
    };
  };
}
