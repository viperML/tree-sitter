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
in
  stdenv.mkDerivation {
    name = nv.pname;
    inherit (nv) src;

    nativeBuildInputs = [
      tree-sitter
      nodejs
      jq
      python3
      importNpmLock.npmConfigHook
    ];

    unpackPhase = ''
      runHook preUnpack

      # tree-sitter checks for the name of the directory
      # (wtf??)
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

    npmDeps = importNpmLock {
      npmRoot = ./tree-sitter-tsx-198d03553f43a45b92ac5d0ee167db3fec6a6fd6;
    };

    npmRebuildFlags = [
      "--ignore-scripts"
    ];

    installPhase = ''
      runHook preInstall

      mkdir -p $out/parser
      for file in *.so; do
        cp -v "$file" $out
        # compatibility with nvim-treesitter
        ln -vsfT "../$file" "$out/parser/$file"
      done

      if [[ "$GRAMMAR_LOCATION" != "null" ]]; then
        popd
      fi

      python3 ${./install.py}

      cp package.json $out

      runHook postInstall
    '';
  }
