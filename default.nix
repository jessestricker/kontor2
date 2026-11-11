let
  sources = import ./npins;
  pkgs = import sources.nixpkgs { };

  kontor = pkgs.callPackage ./packages/kontor { };
in
rec {
  inherit kontor;

  nixpkgs = pkgs;

  user-env = pkgs.buildEnv {
    name = "user-env";
    paths = [ kontor ];
  };
}
