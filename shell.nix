{pkgs ? import <nixpkgs> {}}:
pkgs.stdenv.mkDerivation {
  name = "name";
  nativeBuildInputs = with pkgs; [
    # You guess what it is
    pkg-config
    gcc
    clippy

    # Nix
    nixd
    nixpkgs-fmt
    nixpkgs-lint

    # Rust
    rustc
    cargo
    rustfmt
    rust-analyzer # LSP Server
  ];

  buildInputs = with pkgs; [
    clang
    openssl
    llvmPackages.bintools
  ];

  RUST_BACKTRACE = 1;
  RUSTC_VERSION = builtins.readFile ./rust-toolchain.toml;
  RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
  # LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath (buildInputs ++ nativeBuildInputs);
  LIBCLANG_PATH = pkgs.lib.makeLibraryPath [pkgs.llvmPackages_latest.libclang.lib];
}
