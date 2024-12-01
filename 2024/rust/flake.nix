{
  description = "Advent of Code 2024 Rust Project";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
  };

  outputs =
    { self, nixpkgs }:
    let
      forAllSystems = nixpkgs.lib.genAttrs [
        "x86_64-linux"
        "aarch64-linux"
      ];
    in
    {
      devShells = forAllSystems (system: {
        default =
          let
            pkgs = import nixpkgs {
              inherit system;
            };
          in
          pkgs.rustPlatform.buildRustPackage {
            pname = "advent-of-code-2024";
            version = "0.1.0";

            src = ./.;
            cargoLock.lockFile = ./Cargo.lock;

            nativeBuildInputs = with pkgs; [
              # Additional rust tooling
              rust-analyzer
              rustfmt
              clippy
              llvmPackages.bintools

              just

              cargo-watch
              cargo-generate
              cargo-flamegraph
              cargo-nextest
            ];
          };
      });
    };
}
