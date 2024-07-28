{
  inputs.nixpkgs.url = "github:NixOS/nixpkgs?ref=nixos-unstable";

  outputs = {
    self,
    nixpkgs,
  }: let
    system = "x86_64-linux";
    pkgs = nixpkgs.legacyPackages.${system};

    default = import ./nix {inherit pkgs;};
  in {
    packages.${system} =
      {
        inherit (default) all-grammars;
      }
      // default.grammars;
  };
}
