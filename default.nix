let
  sources = import ./npins;
  pkgs = import sources.nixpkgs { };

  kontor = pkgs.callPackage ./packages/kontor { };

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

  # The Nix channels.
  channels = pkgs.linkFarm "kontor-channels" {
    inherit (sources)
      nixpkgs
      ;
  };

  # The Kontor configuration.
  config = pkgs.linkFarm "kontor-config" {
    inherit user-env channels;
  };
in
{
  inherit
    pkgs
    kontor
    user-env
    channels
    config
    ;
}
