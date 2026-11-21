let
  self = import ./.;
  inherit (self) pkgs;
in
pkgs.mkShell {
  packages = with pkgs; [
    # Nix
    nixd
    nixfmt

    # Python
    ruff
  ];

  inputsFrom = [ self.kontor ];

  # https://github.com/arrterian/nix-env-selector/issues/95
  shellHook = ''
    unset TEMP TMP TEMPDIR TMPDIR
  '';
}
