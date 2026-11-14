let
  sources = import ./npins;
  pkgs = import sources.nixpkgs { };

  kontor = pkgs.callPackage ./packages/kontor { };
in
rec {
  inherit kontor pkgs;

  user-env = pkgs.buildEnv {
    name = "user-env";
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
