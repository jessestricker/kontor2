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
}
