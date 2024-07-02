use std::{io, time::Instant};

use clap::{Parser, ValueEnum};
use gtsp::{
    gtsp::{
        neighborhoods::{SwapNeighborhood, TwoOptNeighborhood},
        GtspProblem,
    },
    localsearch::LocalSearch,
    tabusearch::TabuSearch,
    termination::Termination,
    MetaHeuristic,
};
use rand::{rngs::SmallRng, SeedableRng};

#[derive(Debug, Clone, Copy, ValueEnum)]
enum AlgorithmChoice {
    Local,
    Tabu,
}

#[derive(Debug, Clone, Copy, ValueEnum)]
enum NeighborhoodChoice {
    TwoOpt,
    Swap,
}

#[derive(Debug, Parser)]
struct Cli {
    #[arg(long, default_value_t = 42)]
    seed: u64,
    algorithm: AlgorithmChoice,
    neighborhood: NeighborhoodChoice,
    #[arg(value_parser = parse_termination)]
    termination: Termination,
    #[arg(short, long, value_parser = clap::value_parser!(u32).range(1..))]
    tries: u32,
}

fn parse_termination(s: &str) -> Result<Termination, &'static str> {
    if let Ok(duration) = duration_str::parse(s) {
        Ok(Termination::Timeout(Instant::now() + duration))
    } else if let Ok(n) = s.parse::<usize>() {
        Ok(Termination::Iterations(n))
    } else {
        Err("cannot parse termination")
    }
}

fn main() -> anyhow::Result<()> {
    let args = Cli::parse();

    let stdin = io::stdin().lock();
    let problem = GtspProblem::<i64>::read_from_text(stdin)?;

    let mut rng = SmallRng::seed_from_u64(args.seed);

    let best = (0..args.tries)
        .map(|_| match (args.algorithm, args.neighborhood) {
            (AlgorithmChoice::Local, NeighborhoodChoice::TwoOpt) => {
                LocalSearch::<_, TwoOptNeighborhood>::new().run(&problem, &mut rng)
            }
            (AlgorithmChoice::Local, NeighborhoodChoice::Swap) => {
                LocalSearch::<_, SwapNeighborhood>::new().run(&problem, &mut rng)
            }
            (AlgorithmChoice::Tabu, NeighborhoodChoice::TwoOpt) => {
                TabuSearch::<TwoOptNeighborhood>::new(args.termination).run(&problem, &mut rng)
            }
            (AlgorithmChoice::Tabu, NeighborhoodChoice::Swap) => {
                TabuSearch::<SwapNeighborhood>::new(args.termination).run(&problem, &mut rng)
            }
        })
        .min_by_key(|s| s.weight())
        .unwrap();

    println!("{}", best.weight());

    Ok(())
}
