{ pkgs ? import <nixpkgs> {} }:

pkgs.mkShell {
  buildInputs = with pkgs; [
    ghc
    haskellPackages.ansi-terminal
    haskellPackages.containers
  ];
}