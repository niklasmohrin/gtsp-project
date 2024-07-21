{
  description = "GTSP";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-24.05";
    gtsp-solutions = {
      url = "file+http://www.cs.nott.ac.uk/~dxk/gtsplib/SolutionsText.zip";
      flake = false;
    };
  };

  outputs = { nixpkgs, ... }@inputs:
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

        gtsp-solutions = pkgs.runCommand "gtsp-solutions" { } "${pkgs.unzip}/bin/unzip -d $out ${inputs.gtsp-solutions}";

        r-env = pkgs.rWrapper.override { packages = with pkgs.rPackages; [ ggplot2 dplyr scales ]; };
        plots-script = pkgs.writeShellApplication {
          name = "gtsp-plots";
          runtimeInputs = [ r-env ];
          text = ''
            ${./plots.R} ${gtsp-solutions} "$@"
          '';
        };
        default = plots-script;
      };
    };
}
