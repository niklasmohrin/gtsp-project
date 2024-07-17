use std::marker::PhantomData;

use crate::{termination::Termination, ImprovementHeuristic, Move, MoveNeighborhood, Problem};

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

impl<P, N> ImprovementHeuristic<P> for LocalSearch<N>
where
    P: Problem,
    N: MoveNeighborhood<P>,
{
    fn improve(&mut self, instance: &P, current: P::Solution) -> P::Solution {
        let mut best = current;

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
