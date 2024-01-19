{ rustPlatform, lib, pkg-config, glib, gtk4, gtk4-layer-shell }:
let
  manifest = (lib.importTOML ./Cargo.toml).package;
in
rustPlatform.buildRustPackage {
  inherit (manifest) version;

  pname = manifest.name;
  cargoLock.lockFile = ./Cargo.lock;
  src = lib.cleanSource ./.;

  nativeBuildInputs = [ pkg-config ];
  buildInputs = [ glib gtk4 gtk4-layer-shell ];
}
