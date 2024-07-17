{
  description = "GTSP";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-24.05";
    gtsp-solutions = {
      url = "file+http://www.cs.nott.ac.uk/~dxk/gtsplib/SolutionsText.zip";
      flake = false;
    };
  };

  outputs = { nixpkgs, gtsp-solutions, ... }:
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

        plots =
          let
            fs = pkgs.lib.fileset;
          in
          pkgs.stdenvNoCC.mkDerivation {
            name = "GTSP plots";
            src = fs.toSource { root = ./.; fileset = fs.unions [ ./plots.R ./results.csv ]; };
            buildInputs = with pkgs; [ R rPackages.ggplot2 rPackages.dplyr rPackages.scales unzip ];
            postUnpack = "unzip -d source/solutions ${gtsp-solutions}";
            buildPhase = "R --vanilla -f plots.R";
            installPhase = "cp out.pdf $out";
          };
        default = plots;
      };
    };
}
