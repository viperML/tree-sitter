{
  nv,
  stdenv,
  tree-sitter,
  lib,
  jq,
  python3,
  nodejs,
  importNpmLock,
}: let
  lang = lib.removePrefix "tree-sitter-" nv.pname;
  repoName = builtins.baseNameOf nv.src.url;

  npmRoot = ./${nv.pname}-${nv.version};
  useNpm = builtins.pathExists npmRoot;
in
  stdenv.mkDerivation {
    name = nv.pname;
    inherit (nv) src;

    nativeBuildInputs =
      [
        tree-sitter
        nodejs
        jq
        python3
      ]
      ++ (lib.optionals useNpm [
        importNpmLock.npmConfigHook
      ]);

    inherit useNpm;

    unpackPhase = ''
      runHook preUnpack

      # tree-sitter checks for the name of the directory
      # (wtf??)
      cp --no-preserve=mode -r $src /build/${repoName}
      cd /build/${repoName}
      echo "=> Working in $PWD"

      echo "=> useNpm: $useNpm"

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
      tree-sitter build

      runHook postBuild
    '';

    npmDeps =
      if useNpm
      then
        importNpmLock {
          inherit npmRoot;
        }
      else null;

    npmRebuildFlags = [
      "--ignore-scripts"
    ];

    installPhase = ''
      runHook preInstall

      mkdir -p $out/$GRAMMAR_LOCATION
      for file in *.so; do
        cp -v "$file" $out/$GRAMMAR_LOCATION
      done

      if [[ "$GRAMMAR_LOCATION" != "null" ]]; then
        popd
      fi

      python3 ${./install.py}

      cp package.json $out

      runHook postInstall
    '';
  }
