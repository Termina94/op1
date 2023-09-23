{ pkgs, ... }:

pkgs.callPackage pkgs.rustPlatform.buildRustPackage {
    pname = "op1";
    version = "0.0.1";
    src = ./.;
    cargoBuildFlags = "-p op1";

    cargoLock = {
        lockFile = ./Cargo.lock;
    };

    nativeBuildInputs = with pkgs; [ 
        pkg-config
        alsa-oss
    ];
    buildInputs = with pkgs; [ 
        alsa-lib
    ];

    PKG_CONFIG_PATH = "${pkgs.openssl.dev}/lib/pkgconfig";
}