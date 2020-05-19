with import <nixpkgs> {};

stdenv.mkDerivation {
    name = "rcchat";

    buildInputs = [
      rustc
      cargo
      pkgconfig
      openssl
      gtk3
    ];
}
