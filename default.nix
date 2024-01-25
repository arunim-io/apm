{ rustPlatform, lib }:
let
  manifest = (lib.importTOML ./Cargo.toml).package;
in
rustPlatform.buildRustPackage {
  inherit (manifest) version;

  pname = manifest.name;
  cargoLock.lockFile = ./Cargo.lock;
  src = lib.cleanSource ./.;
}
