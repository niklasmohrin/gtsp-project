use std::{
    io,
    time::{Duration, Instant},
};

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
        ($m: ty) => {
            writer.serialize(Run {
                name: stringify!($m),
                weight: <$m>::new(Termination::Timeout(
                    Instant::now() + Duration::from_millis(100),
                ))
                .run(&problem, &mut rng)
                .weight(),
            })
        };
    }

    for _ in 0..10 {
        run!(LocalSearch::<TwoOptNeighborhood>)?;
        run!(LocalSearch::<SwapNeighborhood>)?;
        run!(TabuSearch::<TwoOptNeighborhood>)?;
        run!(TabuSearch::<SwapNeighborhood>)?;
    }

    Ok(())
}
