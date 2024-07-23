{
  nv,
  meta,
  stdenv,
}: let
  location = meta.location or ".";
in
  stdenv.mkDerivation {
    inherit (nv) pname src;
    version = nv.date;

    buildPhase = ''
      pushd ${location}

      if [[ -f src/scanner.c ]]; then
        $CC -c -Isrc src/scanner.c -o scanner.o
      fi

      $CC -c -Isrc src/parser.c -o parser.o

      $CXX -shared *.o -o _parser

      popd
    '';

    installPhase = ''
      mkdir -p $out
      cp -v ${location}/_parser $out/parser

      if [[ -d queries ]]; then
        cp -vr queries $out
      elif [[ -d ${location}/queries ]]; then
        cp -vr ${location}/queries $out
      fi
    '';
  }
