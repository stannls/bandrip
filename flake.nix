{
  inputs = {
    naersk.url = "github:nix-community/naersk/master";
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    utils.url = "github:numtide/flake-utils";
    nixpkgs-mozilla = {
      url = "github:mozilla/nixpkgs-mozilla";
      flake = false;
    };
  };
  outputs = { self, nixpkgs, utils, naersk, nixpkgs-mozilla }:
    utils.lib.eachDefaultSystem (system:
      let
        pkgs = (import nixpkgs) {
          inherit system;

          overlays = [
            (import nixpkgs-mozilla)
          ];
        };
        naersk-lib = pkgs.callPackage naersk { };
        toolchain = (pkgs.rustChannelOf {
          rustToolchain = ./rust-toolchain.toml;
          sha256 = "sha256-YZARSWuUvkFUPWNVzEa0g6ii8ceU0TqmYMd4YzLxH/U=";
        }).rust;
        naersk' = pkgs.callPackage naersk {
          cargo = toolchain;
          rustc = toolchain;
        };
      in
      {
        defaultPackage = naersk-lib.buildPackage {
		  src = ./.;
          buildInputs = with pkgs; [ openssl pkg-config ];
		};
        devShell = with pkgs; mkShell {
          buildInputs = [ toolchain rustfmt pre-commit rustPackages.clippy pkg-config openssl ];
          RUST_SRC_PATH = rustPlatform.rustLibSrc;
        };
      }
    );
}
