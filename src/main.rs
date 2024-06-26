use std::io;

use gtsp::{
    gtsp::{GtspProblem, Instance, TwoOptNeighborhood},
    localsearch::LocalSearch,
    MetaHeuristic,
};
use rand::{rngs::SmallRng, SeedableRng};

fn main() -> anyhow::Result<()> {
    let stdin = io::stdin().lock();
    let instance = Instance::<i64>::read_from_text(stdin)?;
    let problem = GtspProblem { instance };

    let mut rng = SmallRng::seed_from_u64(42);
    for _ in 0..10 {
        let solution = LocalSearch::<GtspProblem<_>, TwoOptNeighborhood>::run(&problem, &mut rng);
        dbg!(solution);
    }

    Ok(())
}
