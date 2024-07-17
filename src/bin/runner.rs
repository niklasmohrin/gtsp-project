use std::{
    env,
    fs::File,
    io::{self, BufReader},
    time::Duration,
};

use gtsp::{
    gtsp::{
        neighborhoods::{ClusterOptimization, SwapNeighborhood, TwoOptNeighborhood},
        GtspProblem, RandomSolution,
    },
    localsearch::LocalSearch,
    multistart::Multistart,
    tabusearch::TabuSearch,
    termination::Termination,
    ImproveInitial, MetaHeuristic,
};
use rand::{rngs::SmallRng, SeedableRng as _};
use serde_derive::Serialize;

#[derive(Debug, Serialize)]
struct Run<'a> {
    problem: &'a str,
    name: &'a str,
    weight: i64,
}

fn main() -> anyhow::Result<()> {
    let mut rng = SmallRng::seed_from_u64(42);
    let mut writer = csv::Writer::from_writer(io::stdout().lock());

    for path in env::args().skip(1) {
        let problem = GtspProblem::<i64>::read_from_text(BufReader::new(File::open(&path)?))?;

        macro_rules! run {
            ($name: expr, $m: expr) => {
                writer.serialize(Run {
                    problem: &path,
                    name: $name,
                    weight: $m.run(&problem).weight(),
                })
            };
        }

        macro_rules! of_random {
            ($im: expr) => {
                ImproveInitial::new(
                    RandomSolution::new(SmallRng::from_rng(&mut rng).unwrap()),
                    $im,
                )
            };
        }

        for _ in 0..10 {
            run!(
                "MS LS 2-Opt",
                Multistart::new(
                    Termination::after_duration(Duration::from_millis(100)),
                    || of_random!(LocalSearch::<TwoOptNeighborhood>::new(Termination::never()))
                )
            )?;
            run!(
                "MS LS Swap",
                Multistart::new(
                    Termination::after_duration(Duration::from_millis(100)),
                    || of_random!(LocalSearch::<SwapNeighborhood>::new(Termination::never()))
                )
            )?;
            run!(
                "Tabu 2-Opt (L=100)",
                of_random!(TabuSearch::<TwoOptNeighborhood, 100>::new(
                    Termination::after_duration(Duration::from_millis(100))
                ))
            )?;
            run!(
                "Tabu Swap (L=100)",
                of_random!(TabuSearch::<SwapNeighborhood, 100>::new(
                    Termination::after_duration(Duration::from_millis(100))
                ))
            )?;
            run!(
                "Tabu 2-Opt (L=500)",
                of_random!(TabuSearch::<TwoOptNeighborhood, 500>::new(
                    Termination::after_duration(Duration::from_millis(100))
                ))
            )?;
            run!(
                "Tabu Swap (L=500)",
                of_random!(TabuSearch::<SwapNeighborhood, 500>::new(
                    Termination::after_duration(Duration::from_millis(100))
                ))
            )?;
            run!("CO", of_random!(ClusterOptimization))?;
        }
    }

    Ok(())
}
