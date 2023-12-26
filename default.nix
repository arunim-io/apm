{ rustPlatform, lib }: rustPlatform.buildRustPackage {
  pname = "abar";
  version = "0.1.o";
  cargoLock.lockFile = ./Cargo.lock;
  src = lib.cleanSource ./.;
}
