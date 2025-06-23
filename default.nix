# reference: https://dev.to/misterio/how-to-package-a-rust-app-using-nix-3lh3
{pkgs ? import <nixpkgs> {}}: let
  manifest = pkgs.lib.importTOML ./Cargo.toml;
  quotes_file = ./quotes.json;
in
  pkgs.rustPlatform.buildRustPackage {
    pname = "rust_cli_task";
    version = "1.0.0";
    src = pkgs.lib.cleanSource ./.;
    cargoLock.lockFile = ./Cargo.lock;

    postInstall= ''
    mkdir -p $out/share
    cp ${quotes_file} $out/share/quotes.json
    '';

    # Tests require network access. Skipping.
    doCheck = false;

    meta = {
      description = "Fast line-oriented regex search tool, similar to ag and ack";
      homepage = "https://github.com/bahrom04/rust_cli_task";
      license = pkgs.lib.licenses.unlicense;
      maintainers = [];
    };
  }