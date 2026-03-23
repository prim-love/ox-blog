{
  inputs = {
    flake-utils.url = "github:numtide/flake-utils";
    nixpkgs.url     = "github:NixOS/nixpkgs/nixpkgs-unstable";
  };

  outputs = { self, nixpkgs, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let pkgs = import nixpkgs { inherit system; };
          m    = (pkgs.lib.importTOML ./Cargo.toml).package;

          pkg  = pkgs.rustPlatform.buildRustPackage {
            cargoLock.lockFile = ./Cargo.lock;
            version            = m.version;
            name               = m.name;
            src                = self;
          };
      in rec {
        packages.default = pkg;

        export           = { name, src }: pkgs.stdenv.mkDerivation {
          inherit name src;

          buildPhase = ''
          mkdir $out

          ${pkg}/bin/${m.name} $src $out
          '';
        };
      });
}
