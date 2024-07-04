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
    multistart::Multistart,
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

    Ok(())
}
