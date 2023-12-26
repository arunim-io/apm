{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    systems.url = "github:nix-systems/default";
    flake-parts = {
      url = "github:hercules-ci/flake-parts";
      inputs.nixpkgs-lib.follows = "nixpkgs";
    };
  };

  outputs = inputs@{ flake-parts, systems, nixpkgs, ... }: flake-parts.lib.mkFlake { inherit inputs; } {
    systems = import systems;

    perSystem = { pkgs, system, config, ... }: {
      packages = {
        abar = config.packages.default;
        default = with pkgs; rustPlatform.buildRustPackage {
          pname = "abar";
          version = "0.1.o";
          cargoLock.lockFile = ./Cargo.lock;
          src = lib.cleanSource ./.;
        };
      };
      devShells.default = with pkgs; mkShell {
        inputsFrom = [ config.packages.default ];
        nativeBuildInputs = [ rustc cargo gcc rustfmt clippy ];
        RUST_SRC_PATH = "${rustPlatform.rustLibSrc}";
      };
    };
  };
}
