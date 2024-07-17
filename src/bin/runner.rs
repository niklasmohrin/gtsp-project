use std::{
    env,
    fs::File,
    io::{self, BufReader},
    time::Duration,
};

use itertools::Itertools as _;

use gtsp::{
    gtsp::{
        neighborhoods::{SwapNeighborhood, TwoOptNeighborhood},
        GtspProblem,
    },
    localsearch::LocalSearch,
    multistart::Multistart,
    tabusearch::TabuSearch,
    termination::Termination,
    MetaHeuristic,
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
            ($m: expr) => {
                writer.serialize(Run {
                    problem: &path,
                    name: &stringify!($m).split_whitespace().join(" "),
                    weight: $m.run(&problem, &mut rng).weight(),
                })
            };
        }

        for _ in 0..10 {
            run!(Multistart::new(
                Termination::after_duration(Duration::from_millis(100)),
                || LocalSearch::<TwoOptNeighborhood>::new(Termination::never())
            ))?;
            run!(Multistart::new(
                Termination::after_duration(Duration::from_millis(100)),
                || LocalSearch::<SwapNeighborhood>::new(Termination::never())
            ))?;
            run!(TabuSearch::<TwoOptNeighborhood, 100>::new(
                Termination::after_duration(Duration::from_millis(100))
            ))?;
            run!(TabuSearch::<SwapNeighborhood, 100>::new(
                Termination::after_duration(Duration::from_millis(100))
            ))?;
            run!(TabuSearch::<TwoOptNeighborhood, 500>::new(
                Termination::after_duration(Duration::from_millis(100))
            ))?;
            run!(TabuSearch::<SwapNeighborhood, 500>::new(
                Termination::after_duration(Duration::from_millis(100))
            ))?;
        }
    }

    Ok(())
}
