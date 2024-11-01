{
  pkgs ? import <nixpkgs> { },
}:

pkgs.mkShell {
  nativeBuildInputs = with pkgs; [
    # Dependencies that should only exist in the build environment.
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
}
