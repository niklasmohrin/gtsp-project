use std::io;

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
use rand::{rngs::SmallRng, SeedableRng as _};
use serde_derive::Serialize;

#[derive(Debug, Serialize)]
struct Run {
    name: &'static str,
    weight: i64,
}

fn main() -> anyhow::Result<()> {
    let stdin = io::stdin().lock();
    let problem = GtspProblem::<i64>::read_from_text(stdin)?;

    let mut rng = SmallRng::seed_from_u64(42);

    let mut writer = csv::Writer::from_writer(io::stdout().lock());

    macro_rules! run {
        ($m: expr) => {
            writer.serialize(Run {
                name: stringify!($m),
                weight: $m.run(&problem, &mut rng).weight(),
            })
        };
    }

    let termination = Termination::Iterations(5);

    for _ in 0..10 {
        run!(LocalSearch::<TwoOptNeighborhood>::new())?;
        run!(LocalSearch::<SwapNeighborhood>::new())?;
        run!(TabuSearch::<SwapNeighborhood>::new(termination))?;
        run!(TabuSearch::<SwapNeighborhood>::new(termination))?;
    }

    Ok(())
}
