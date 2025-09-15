{}:
with import <nixpkgs> {};
  mkShell rec {
    packages = [
      rustup
      rustc
      cargo
      rustfmt
      rust-analyzer
    ];
  }
