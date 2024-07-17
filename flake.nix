{
  description = "GTSP";

  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixos-24.05";

  outputs = { nixpkgs, ... }:
    let
      system = "x86_64-linux";
      pkgs = import nixpkgs { inherit system; };
    in
    {
      packages.${system} = rec {
        runner = pkgs.rustPlatform.buildRustPackage {
          pname = "gtsp-runner";
          version = "0.0.1";
          src = ./.;
          cargoLock.lockFile = ./Cargo.lock;
          cargoBuildFlags = [ "--bin" "runner" ];
        };

        plots = pkgs.stdenvNoCC.mkDerivation {
          name = "GTSP plots";
          src = ./.;
          buildInputs = with pkgs; [ R rPackages.ggplot2 rPackages.dplyr rPackages.scales ];
          buildPhase = "R --vanilla -f plots.R";
          installPhase = "cp out.pdf $out";
        };
        default = plots;
      };
    };
}
