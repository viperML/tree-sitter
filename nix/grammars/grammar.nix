{
  nv,
  stdenv,
  nodejs,
  tree-sitter,
  lib,
  jq,
}: let
  lang = lib.removePrefix "tree-sitter-" nv.pname;
in
  stdenv.mkDerivation {
    name = nv.pname;
    inherit (nv) src;

    nativeBuildInputs = [
      tree-sitter
      nodejs
      jq
    ];

    unpackPhase = ''
      runHook preUnpack
      cp --no-preserve=mode -r $src /build/${nv.pname}
      cd /build/${nv.pname}
      runHook postUnpack
    '';

    buildPhase = ''
      runHook preBuild

      export HOME=$PWD

      export GRAMMAR_LOCATION="$(jq -r '.${lang}.install_info.location' < ${./meta.json})"
      export GRAMMAR_REQUIRES_GENERATE="$(jq -r '.${lang}.install_info.requires_generate_from_grammar' < ${./meta.json})"

      if [[ "$GRAMMAR_LOCATION" != "null" ]]; then
        pushd "$GRAMMAR_LOCATION"
      fi

      if [[ "$GRAMMAR_REQUIRES_GENERATE" = "true" ]]; then
        echo "=> Generating grammar"
        tree-sitter generate
      fi

      echo "=> Building grammar"
      tree-sitter build -o ${lang}.so

      runHook postBuild
    '';

    installPhase = ''
      runHook preInstall

      mkdir -p $out/parser
      cp -v *.so $out/parser

      if [[ "$GRAMMAR_LOCATION" != "null" ]]; then
        popd
      fi

      mkdir -p $out/queries

      # TODO: install queries by reading the package.json

      runHook postInstall
    '';
  }
