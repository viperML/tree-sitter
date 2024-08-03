{
  rustPlatform,
  callPackages,
}:
let
  nv = (callPackages ./generated.nix {}).tree-sitter;
in
rustPlatform.buildRustPackage {
  pname = "tree-sitter";
  version = nv.date;

  inherit (nv) src;
  cargoLock.lockFile = nv.cargoLock."Cargo.lock".lockFile;

  doCheck = false;
}
