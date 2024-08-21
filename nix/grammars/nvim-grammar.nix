{
  nv,
  stdenv,
  tree-sitter,
  lib,
  jq,
  python3,
  nodejs,
}: let
  lang = lib.removePrefix "tree-sitter-" nv.pname;
  repoName = builtins.baseNameOf nv.src.url;
in
  stdenv.mkDerivation {
    name = nv.pname;
    inherit (nv) src;

    nativeBuildInputs = [
      tree-sitter
      nodejs
      jq
      python3
    ];

    unpackPhase = ''
      runHook preUnpack

      # tree-sitter checks for the name of the directory
      # (wtf??)
      cp --no-preserve=mode -r $src /build/${repoName}
      cd /build/${repoName}
      echo "=> Working in $PWD"

      runHook postUnpack
    '';

    buildPhase = ''
      runHook preBuild

      export HOME=$PWD

      export GRAMMAR_LOCATION="$(jq -r '.${lang}.install_info.location' < ${./meta.json})"
      export GRAMMAR_REQUIRES_GENERATE="$(jq -r '.${lang}.install_info.requires_generate_from_grammar' < ${./meta.json})"

      if [[ "$GRAMMAR_LOCATION" == "null" ]]; then
        GRAMMAR_LOCATION=""
      fi

      pushd "$GRAMMAR_LOCATION"

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
      cp *.so $out/parser

      runHook postInstall
    '';
  }
