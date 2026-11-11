{
  lib,
  rustPlatform,
}:
let
  cargoToml = lib.importTOML ./Cargo.toml;
in
rustPlatform.buildRustPackage {
  pname = cargoToml.package.name;
  version = cargoToml.package.version;

  src = lib.sources.sourceByGlobs ./. [
    "Cargo.toml"
    "Cargo.lock"
    "src/**"
  ];
  cargoLock = {
    lockFile = ./Cargo.lock;
  };
}
