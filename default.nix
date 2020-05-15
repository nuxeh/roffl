with import <nixpkgs> {};

stdenv.mkDerivation {
    name = "roffl";

    buildInputs = [
      rustc
      cargo
      pkgconfig
      openssl
      gtk3
    ];
}
