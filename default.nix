{ rustPlatform, lib, libxkbcommon, libGL, wayland, xorg }:
let
  manifest = (lib.importTOML ./Cargo.toml).package;
in
rustPlatform.buildRustPackage {
  inherit (manifest) version;

  pname = manifest.name;
  cargoLock.lockFile = ./Cargo.lock;
  src = lib.cleanSource ./.;

  buildInputs = [
    libxkbcommon
    libGL
    wayland
    xorg.libXcursor
    xorg.libXrandr
    xorg.libXi
    xorg.libX11
  ];
}
