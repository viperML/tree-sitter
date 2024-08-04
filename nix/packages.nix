{
  member,
  rustPlatform,
  lib,
  makeWrapper,

  ts-grammar-path,
}: let
  r = ../.;

  cargoToml = builtins.fromTOML (builtins.readFile (r + "/Cargo.toml"));
  members = cargoToml.workspace.members;
in
  rustPlatform.buildRustPackage {
    name = member;

    src = lib.fileset.toSource {
      root = r;
      fileset = lib.fileset.intersection (lib.fileset.fromSource (lib.sources.cleanSource r)) (
        lib.fileset.unions ([
            (r + "/Cargo.toml")
            (r + "/Cargo.lock")
          ]
          ++ (map (m: r + "/${m}") members))
      );
    };

    cargoLock.lockFile = r + "/Cargo.lock";

    buildAndTestSubdir = member;

    nativeBuildInputs = [
      makeWrapper
    ];

    postInstall = ''
      for file in $out/bin/*; do
        wrapProgram $out/bin/* \
          --set-default TS_GRAMMAR_PATH '${ts-grammar-path}'
      done
    '';
  }
