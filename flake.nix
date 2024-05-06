{
  description = "Nix flake helper";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixpkgs-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url = "github:numtide/flake-utils";


  };

  outputs = { self, nixpkgs, rust-overlay, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs { inherit system overlays; };
        cargoToml = builtins.fromTOML (builtins.readFile ./Cargo.toml);
        msrv = cargoToml.package.rust-version;

        mkPackage = rust: (pkgs.makeRustPlatform {
          cargo = rust;
          rustc = rust;
        }).buildRustPackage {
          inherit (cargoToml.package) name version;
          src = ./.;
          cargoLock.lockFile = ./Cargo.lock;
          nativeBuildInputs = [ rust ];
          dontUseCargoParallelTests = true;
        };

        purgaPackage = mkPackage pkgs.rust-bin.stable.latest.minimal;
      in
      {
        packages.default = purgaPackage;
        overlays.default = _: _: {
          purga = purgaPackage;
        };
      }
    );
}
