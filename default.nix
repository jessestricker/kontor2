let
  sources = import ./npins;
  pkgs = import sources.nixpkgs { };

  kontor = pkgs.callPackage ./packages/kontor { };
in
rec {
  inherit kontor pkgs;

  # The user environment.
  user-env = pkgs.buildEnv {
    name = "kontor-user-env";
    paths =
      (with pkgs; [
        git
        micro
        nixd
        nixfmt
        npins
        shellcheck
        shfmt
      ])
      ++ [ kontor ];
  };
}
