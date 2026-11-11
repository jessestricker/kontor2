let
  sources = import ./npins;
  pkgs = import sources.nixpkgs { };
in
{
  nixpkgs = pkgs;
}
