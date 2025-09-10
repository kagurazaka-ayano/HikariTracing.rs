{}:
with import <nixpkgs> {};
  stdenv.mkDerivation rec {
    pname = "hikaritracing-rs";
    name = "hikaritracing-rs";
    nativeBuildInputs = [
    ];
    buildInputs = [
      rustup
      rustc
      cargo
      rustfmt
      rust-analyzer
    ];
  }
