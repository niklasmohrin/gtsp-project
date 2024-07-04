use std::marker::PhantomData;

use rand::Rng;

use crate::{termination::Termination, MetaHeuristic, Move, MoveNeighborhood, Problem};

pub struct LocalSearch<N> {
    termination: Termination,
    _n: PhantomData<N>,
}

impl<N> LocalSearch<N> {
    pub fn new(termination: Termination) -> Self {
        Self {
            termination,
            _n: PhantomData,
        }
    }
}

impl<P, N> MetaHeuristic<P> for LocalSearch<N>
where
    P: Problem,
    N: MoveNeighborhood<P>,
{
    fn run(mut self, instance: &P, rng: impl Rng) -> P::Solution {
        let mut best = instance.make_intial_solution(rng);

        while !self.termination.should_terminate() {
            let Some(new_best) = N::moves_iter(instance, &best)
                .max_by_key(|m| m.score_increase())
                .filter(|m| m.is_improving())
                .map(|m| m.into_solution())
            else {
                break;
            };
            best = new_best;

            self.termination.iteration();
        }

        best
    }
}
