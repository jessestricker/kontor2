let
  sources = import ./npins;
  pkgs = import sources.nixpkgs { };
in
{
  kontor = pkgs.callPackage ./packages/kontor {};
  nixpkgs = pkgs;
}
