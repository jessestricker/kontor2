let
  sources = import ./npins;
  pkgs = import sources.nixpkgs { };

  # The kontor CLI.
  kontor = pkgs.callPackage ./packages/kontor { };

  # The Nix configuration file.
  nix-conf = pkgs.writeTextFile {
    # See https://nix.dev/manual/nix/2.36/command-ref/conf-file.html.
    name = "kontor-nix-conf";
    text = ''
      experimental-features = nix-command
      nix-path = nixpkgs=${sources.nixpkgs}
    '';
    destination = "/etc/nix/nix.conf";
  };

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
      ++ [
        kontor
        nix-conf
      ];
  };
in
{
  inherit
    pkgs
    kontor
    nix-conf
    user-env
    ;
}
