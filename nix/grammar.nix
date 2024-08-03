{
  nv,
  meta,
  stdenv,
  nodejs,
  tree-sitter,
  lib,
  nvim-treesitter,
}: let
  lang = lib.removePrefix "tree-sitter-" nv.pname;
  location = meta.location or ".";
in
  stdenv.mkDerivation {
    name = nv.pname;
    inherit (nv) src;

    nativeBuildInputs = [
      tree-sitter
      nodejs
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

      ${
        if meta.install_info ? location
        then "pushd ${meta.install_info.location}"
        else ""
      }

      ${
        if meta.install_info ? requires_generate_from_grammar && meta.install_info.requires_generate_from_grammar
        # if true
        then ''
          echo "=> Generating grammar"
          tree-sitter generate
        ''
        else ""
      }

      echo "=> Building grammar"
      tree-sitter build -o ${lib.removePrefix "tree-sitter-" nv.pname}.so

      runHook postBuild
    '';

    installPhase = ''
      runHook preInstall
      mkdir -p $out/parser
      cp -v *.so $out/parser

      mkdir -p $out/queries

      if [[ -d ${nvim-treesitter}/queries/${lang} ]]; then
        cp -vr ${nvim-treesitter}/queries/${lang} $out/queries
      fi

      runHook postInstall
    '';

    checkPhase = ''
      runHook preCheck
      tree-sitter test
      runHook postCheck
    '';
  }
