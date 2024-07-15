{
  description = "GTSP";

  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixos-24.05";

  outputs = { nixpkgs, ... }:
    let
      system = "x86_64-linux";
      pkgs = import nixpkgs { inherit system; };
    in
    {
      packages.${system}.default = pkgs.stdenvNoCC.mkDerivation {
        name = "GTSP plots";
        src = ./.;
        buildInputs = with pkgs; [ R rPackages.ggplot2 ];
        buildPhase = "R --vanilla -f plots.R";
        installPhase = "cp Rplots.pdf $out";
      };
    };
}
