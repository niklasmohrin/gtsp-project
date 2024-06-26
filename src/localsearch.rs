use std::marker::PhantomData;

use rand::Rng;

use crate::{MetaHeuristic, Move, MoveNeighborhood, Problem};

pub struct LocalSearch<P, N> {
    _p: PhantomData<P>,
    _n: PhantomData<N>,
}

impl<P, N> MetaHeuristic<P> for LocalSearch<P, N>
where
    P: Problem,
    N: MoveNeighborhood<P>,
{
    fn run(instance: &P, rng: impl Rng) -> P::Solution {
        let mut best = instance.make_intial_solution(rng);

        while let Some(new_best) = N::moves_iter(instance, &best)
            .max_by_key(|m| m.score_increase())
            .filter(|m| m.is_improving())
            .map(|m| m.into_solution())
        {
            best = new_best;
        }

        best
    }
}
