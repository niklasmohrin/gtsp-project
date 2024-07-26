# Meta Heuristics for GTSP

This repository contains the code for my final project of the "Automated Decision Making" course of the summer term 2024 at Unimore.

The goal is to compare different neighborhoods for the Generalized Traveling Salesperson Problem (GTSP) for the Localsearch and Tabusearch meta heuristics. The neighborhood definitions are taken from _Gutin and Karapetyan (2009)_ where they are used as part of a GTSP-specific algorithm. Since their algorithm uses a cycling scheme with multiple neighborhoods and the additional step of _Cluster Optimization_, these two techniques are also investigated.

A secondary goal is to program the meta heuristics as generic algorithms using Rust traits. These traits are defined in `src/lib.rs`. The details for GTSP and the neighborhoods are located in the `src/gtsp` subdirectory.

## Running

The following will assume that you have downloaded the instances and solutions from the [GTSP instances library](https://www.cs.nott.ac.uk/~pszdk/gtsp.html) in the text format and placed them in the `instances/` and `solutions/` subdirectory.

To build the code, either `nix` or a Rust installation is needed. The experiment runner can then be built using `nix build .#runner` or `cargo build --release --bin runner` respectively. To run the experiment, substitute "build" with "run" and add the path to each input instance you would like to include as an argument (example: `nix run .#runner -- instances/11berlin52.txt instances/26bier127.txt`). The results will be printed to the terminal as CSV.

To generate plots of the results, either `nix` or an R installation with some libraries is needed (see `plots.R`). Build the plots using `nix run .#plots-script results.csv plots.pdf` or `./plots.R solutions/ results.csv plots.pdf` respectively.

## Bibliography

- Gutin, G., Karapetyan, D. (2009). A memetic algorithm for the generalized traveling salesman problem. Natural Computing, 9(1), 47-60.
