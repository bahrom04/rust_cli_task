# reference: https://dev.to/misterio/how-to-package-a-rust-app-using-nix-3lh3
{
  pkgs ? import <nixpkgs> {},
  fenix ? import <fenix> {},
}: let
  manifest = pkgs.lib.importTOML ./Cargo.toml;
  quotes_file = ./data/quotes.json;
  test_quotes_file = ./data/test.json;

  # Rust Toolchain via fenix
  toolchain = fenix.packages.${pkgs.system}.fromToolchainFile {
    file = ./rust-toolchain.toml;

    # Don't worry, if you need sha256 of your toolchain,
    # just run `nix build` and copy paste correct sha256.
    sha256 = "sha256-0Hcko7V5MUtH1RqrOyKQLg0ITjJjtyRPl2P+cJ1p1cY=";
  };
in
  pkgs.rustPlatform.buildRustPackage {
    pname = "rust_cli_task";
    version = "1.0.0";
    src = pkgs.lib.cleanSource ./.;
    cargoLock.lockFile = ./Cargo.lock;

    # Compile time dependencies
    nativeBuildInputs = with pkgs; [
      # GCC toolchain
      gcc
      gnumake
      pkg-config

      # LLVM toolchain
      cmake
      llvmPackages.llvm
      llvmPackages.clang

      # Rust
      toolchain

      # Other compile time dependencies
      # here
    ];

    postInstall = ''
      mkdir -p $out/data
      cp ${quotes_file} $out/data/quotes.json
      cp ${test_quotes_file} $out/data/test.json
    '';
    # Tests require network access. Skipping.
    doCheck = true;

    meta = {
      description = "Fast line-oriented regex search tool, similar to ag and ack";
      homepage = "https://github.com/bahrom04/rust_cli_task";
      license = pkgs.lib.licenses.unlicense;
      maintainers = [];
    };
  }
