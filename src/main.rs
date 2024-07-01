use std::io;

use gtsp::{
    gtsp::{
        neighborhoods::{SwapNeighborhood, TwoOptNeighborhood},
        GtspProblem,
    },
    localsearch::LocalSearch,
    MetaHeuristic,
};
use rand::{rngs::SmallRng, SeedableRng};

fn main() -> anyhow::Result<()> {
    let stdin = io::stdin().lock();
    let problem = GtspProblem::<i64>::read_from_text(stdin)?;

    let mut rng = SmallRng::seed_from_u64(42);
    let best2 = (0..10)
        .map(|_| LocalSearch::<GtspProblem<_>, TwoOptNeighborhood>::run(&problem, &mut rng))
        .min_by_key(|s| s.weight());
    let best_swap = (0..10)
        .map(|_| LocalSearch::<GtspProblem<_>, SwapNeighborhood>::run(&problem, &mut rng))
        .min_by_key(|s| s.weight());

    dbg!(best2, best_swap);

    Ok(())
}
