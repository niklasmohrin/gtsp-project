use std::marker::PhantomData;

use rand::Rng;

use crate::{MetaHeuristic, Move, MoveNeighborhood, Problem};

pub struct LocalSearch<N> {
    _n: PhantomData<N>,
}

impl<N> LocalSearch<N> {
    pub fn new() -> Self {
        Self { _n: PhantomData }
    }
}

impl<P, N> MetaHeuristic<P> for LocalSearch<N>
where
    P: Problem,
    N: MoveNeighborhood<P>,
{
    fn run(self, instance: &P, rng: impl Rng) -> P::Solution {
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
