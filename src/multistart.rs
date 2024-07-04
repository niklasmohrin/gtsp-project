use std::iter;

use rand::Rng;

use crate::{termination::Termination, MetaHeuristic, Problem};

pub struct Multistart<F> {
    termination: Termination,
    factory: F,
}

impl<F> Multistart<F> {
    pub fn new(termination: Termination, factory: F) -> Self {
        Self {
            termination,
            factory,
        }
    }
}

impl<P, M, F> MetaHeuristic<P> for Multistart<F>
where
    P: Problem,
    M: MetaHeuristic<P>,
    F: FnMut() -> M,
{
    fn run(mut self, instance: &P, mut rng: impl Rng) -> P::Solution {
        let mut solutions = iter::repeat_with(|| (self.factory)().run(instance, &mut rng));

        let mut best = solutions.next().unwrap();

        while !self.termination.should_terminate() {
            let next = solutions.next().unwrap();
            if P::score(&next) > P::score(&best) {
                best = next;
            }
        }

        best
    }
}
