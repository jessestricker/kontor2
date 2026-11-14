let
  pkgs = (import ./.).nixpkgs;
in
pkgs.mkShell {
  packages = with pkgs; [
    cargo
    clippy
    rustc
    rustfmt
  ];

  RUST_SRC_PATH = "${pkgs.rustPlatform.rustLibSrc}";

  # https://github.com/arrterian/nix-env-selector/issues/95
  shellHook = ''
    unset TEMP TMP TEMPDIR TMPDIR
  '';
}
