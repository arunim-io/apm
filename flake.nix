{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    systems.url = "github:nix-systems/default";
    flake-parts = {
      url = "github:hercules-ci/flake-parts";
      inputs.nixpkgs-lib.follows = "nixpkgs";
    };
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  nixConfig = {
    extra-substituters = [ "https://nix-community.cachix.org" ];
    extra-trusted-public-keys = [ "nix-community.cachix.org-1:mB9FSh9qf2dCimDSUo8Zy7bkq5CX+/rkCWyvRCYg3Fs=" ];
  };

  outputs = inputs: inputs.flake-parts.lib.mkFlake { inherit inputs; } {
    systems = import inputs.systems;

    perSystem = { pkgs, system, config, inputs', ... }:
      let
        inherit (inputs'.fenix.packages.complete) cargo rustc rust-src;
        rustPlatform = pkgs.makeRustPlatform { inherit cargo rustc; };
      in
      {
        packages = {
          default = pkgs.callPackage ./default.nix { inherit rustPlatform; };
          apm = config.packages.default;
        };

        devShells.default = with pkgs; mkShell {
          inputsFrom = [ config.packages.default ];
          RUST_SRC_PATH = "${rust-src}/lib/rustlib/src/rust/library";
        };
      };
  };
}
