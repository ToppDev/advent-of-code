{ callPackage,
  rust-analyzer, rustfmt, clippy, llvmPackages,
  just,
  cargo-watch, cargo-generate, cargo-flamegraph, cargo-nextest,
}:

let
  mainPkg = callPackage ./default.nix { };
in
mainPkg.overrideAttrs (oa: {
  nativeBuildInputs = [
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

    # libGL libGLU # OpenGL
    # vulkan-loader # Vulkan

    # xorg.libxcb xorg.libXcursor xorg.libXi xorg.libXrandr # X11
    # libxkbcommon wayland # Wayland
    # trunk # WASM
  ] ++ (oa.nativeBuildInputs or [ ]);
})
