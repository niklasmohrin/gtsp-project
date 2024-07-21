use std::{
    env,
    fs::File,
    io::{self, BufReader},
    time::{Duration, Instant},
};

use gtsp::{
    chain::Chain,
    gtsp::{
        neighborhoods::{InsertsNeighborhood, SwapNeighborhood, TwoOptNeighborhood},
        ClusterOptimization, GtspProblem, RandomSolution,
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
                        let e = start.elapsed();
                        let over = e.saturating_sub(d);
                        eprintln!(
                            "  {} took {e:?} ({over:?} over the planned duration of {d:?}){}",
                            $name,
                            (if over > d / 10 {
                                " ** THAT IS LONG **"
                            } else {
                                ""
                            })
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

        macro_rules! run_ms {
            ($name: expr, $neigh: ty) => {
                run!(
                    concat!("MS LS ", $name),
                    Multistart::new(Termination::after_duration(d), || of_random!(
                        LocalSearch::<$neigh>::new(Termination::never())
                    ))
                )?;
                run!(
                    concat!("MS LS ", $name, " with CO"),
                    Multistart::new(Termination::after_duration(d), || of_random!(with_co!(
                        LocalSearch::<$neigh>::new(Termination::never())
                    )))
                )?;
            };
        }
        macro_rules! run_tabu {
            ($name: expr, $neigh: ty) => {
                run_tabu!($name, $neigh, 100);
                run_tabu!($name, $neigh, 500);
            };
            ($name: expr, $neigh: ty, $len: expr) => {
                run!(
                    concat!("Tabu ", $name, " (L=", $len, ")"),
                    of_random!(TabuSearch::<$neigh, $len>::new(
                        Termination::after_duration(d)
                    ))
                )?;
                run!(
                    concat!("Tabu ", $name, " (L=", $len, ") with CO"),
                    of_random!(with_co!(TabuSearch::<$neigh, $len>::new(
                        Termination::after_duration(d)
                    )))
                )?;
            };
        }

        macro_rules! run_all {
            ($name: expr, $neigh: ty) => {
                run_ms!($name, $neigh);
                run_tabu!($name, $neigh);
            };
        }

        for _ in 0..10 {
            run_all!("2-Opt", TwoOptNeighborhood);
            run_all!("Swap", SwapNeighborhood);
            run_tabu!("Inserts", InsertsNeighborhood);
        }
    }

    Ok(())
}
