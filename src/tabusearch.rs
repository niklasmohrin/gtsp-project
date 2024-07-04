use std::{collections::VecDeque, marker::PhantomData};

use rand::prelude::Rng;

use crate::{
    termination::{self, Termination},
    MetaHeuristic, Move, Neighborhood, Problem,
};

pub struct TabuSearch<N, const L: usize> {
    termination: Termination,
    _n: PhantomData<N>,
}

impl<N, const L: usize> TabuSearch<N, L> {
    pub fn new(termination: Termination) -> Self {
        Self {
            termination,
            _n: PhantomData,
        }
    }
}

impl<P, N, const L: usize> MetaHeuristic<P> for TabuSearch<N, L>
where
    P: Problem,
    P::Solution: Clone + PartialEq,
    N: Neighborhood<P>,
{
    fn run(mut self, instance: &P, rng: impl Rng) -> <P as Problem>::Solution {
        let mut best = instance.make_intial_solution(rng);
        let mut tabu_list = VecDeque::with_capacity(L + 1);
        tabu_list.push_back(best.clone());
        while !self.termination.should_terminate() {
            let Some(best_neighbor) = N::neighbors_iter(instance, &tabu_list.back().unwrap())
                .filter(|s| !tabu_list.contains(s))
                .max_by_key(|s| P::score(s))
            else {
                break;
            };

            if P::score(&best_neighbor) > P::score(&best) {
                best = best_neighbor.clone();
            }

            tabu_list.push_back(best_neighbor);
            if tabu_list.len() > L {
                tabu_list.pop_front();
            }
            self.termination.iteration();
        }

        best
    }
}
