use std::{
    env,
    fs::File,
    io::{self, BufReader},
    time::{Duration, Instant},
};

use gtsp::{
    chain::Chain,
    gtsp::{
        neighborhoods::{
            ClusterOptimization, InsertsNeighborhood, SwapNeighborhood, TwoOptNeighborhood,
        },
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
        let suffix = if problem.is_symmetric() {
            ""
        } else {
            " (asymm.)"
        };
        let problem_name = path + suffix;
        eprintln!("Problem: {problem_name}");

        let d = Duration::from_millis(100);

        macro_rules! run {
            ($name: expr, $m: expr) => {
                writer.serialize(Run {
                    problem: &problem_name,
                    name: $name,
                    weight: {
                        let start = Instant::now();
                        let res = $m.run(&problem).weight();
                        eprintln!(
                            "  {} took {:?} over the planned duration of {:?}",
                            $name,
                            start.elapsed() - d,
                            d
                        );
                        res
                    },
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

        macro_rules! with_co {
            ($im: expr) => {
                Chain::new($im, ClusterOptimization)
            };
        }

        for _ in 0..10 {
            run!(
                "MS LS 2-Opt",
                Multistart::new(Termination::after_duration(d), || of_random!(
                    LocalSearch::<TwoOptNeighborhood>::new(Termination::never())
                ))
            )?;
            run!(
                "MS LS 2-Opt with CO",
                Multistart::new(Termination::after_duration(d), || of_random!(with_co!(
                    LocalSearch::<TwoOptNeighborhood>::new(Termination::never())
                )))
            )?;
            run!(
                "MS LS Swap",
                Multistart::new(Termination::after_duration(d), || of_random!(
                    LocalSearch::<SwapNeighborhood>::new(Termination::never())
                ))
            )?;
            run!(
                "MS LS Swap with CO",
                Multistart::new(Termination::after_duration(d), || of_random!(with_co!(
                    LocalSearch::<SwapNeighborhood>::new(Termination::never())
                )))
            )?;
            run!(
                "Tabu 2-Opt (L=100)",
                of_random!(TabuSearch::<TwoOptNeighborhood, 100>::new(
                    Termination::after_duration(d)
                ))
            )?;
            run!(
                "Tabu 2-Opt (L=100) with CO",
                of_random!(with_co!(TabuSearch::<TwoOptNeighborhood, 100>::new(
                    Termination::after_duration(d)
                )))
            )?;
            run!(
                "Tabu Swap (L=100)",
                of_random!(TabuSearch::<SwapNeighborhood, 100>::new(
                    Termination::after_duration(d)
                ))
            )?;
            run!(
                "Tabu Swap (L=100) with CO",
                of_random!(with_co!(TabuSearch::<SwapNeighborhood, 100>::new(
                    Termination::after_duration(d)
                )))
            )?;
            run!(
                "Tabu 2-Opt (L=500)",
                of_random!(TabuSearch::<TwoOptNeighborhood, 500>::new(
                    Termination::after_duration(d)
                ))
            )?;
            run!(
                "Tabu 2-Opt (L=500) with CO",
                of_random!(with_co!(TabuSearch::<TwoOptNeighborhood, 500>::new(
                    Termination::after_duration(d)
                )))
            )?;
            run!(
                "Tabu Swap (L=500)",
                of_random!(TabuSearch::<SwapNeighborhood, 500>::new(
                    Termination::after_duration(d)
                ))
            )?;
            run!(
                "Tabu Swap (L=500) with CO",
                of_random!(with_co!(TabuSearch::<SwapNeighborhood, 500>::new(
                    Termination::after_duration(d)
                )))
            )?;
            run!(
                "Tabu Inserts (L=500)",
                of_random!(TabuSearch::<InsertsNeighborhood, 500>::new(
                    Termination::after_duration(d)
                ))
            )?;
        }
    }

    Ok(())
}
