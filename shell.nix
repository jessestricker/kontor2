let
  self = import ./.;
  inherit (self) pkgs;
in
pkgs.mkShell {
  packages = with pkgs; [
    # Nix
    nixd
    nixfmt

    # Rust
    clippy
    rust-analyzer
    rustfmt
  ];

  inputsFrom = [
    self.kontor
  ];

  # Tells rust-analyzer where to find Rust's stdlib source.
  RUST_SRC_PATH = "${pkgs.rustPlatform.rustLibSrc}";

  # https://github.com/arrterian/nix-env-selector/issues/95
  shellHook = ''
    unset TEMP TMP TEMPDIR TMPDIR
  '';
}
