use crate::{termination::Termination, ImprovementHeuristic, Neighborhood, Problem};

pub struct Cycle<P> {
    heuristics: Vec<Box<dyn ImprovementHeuristic<P>>>,
    termination: Termination,
}

impl<P> Cycle<P> {
    pub fn new(
        heuristics: impl IntoIterator<Item = Box<dyn ImprovementHeuristic<P>>>,
        termination: Termination,
    ) -> Self {
        Self {
            heuristics: heuristics.into_iter().collect(),
            termination,
        }
    }
}

impl<P> ImprovementHeuristic<P> for Cycle<P>
where
    P: Problem,
    P::Solution: Clone,
{
    fn improve(&mut self, instance: &P, mut current: P::Solution) -> P::Solution {
        let mut i = 0;
        while !self.termination.should_terminate() && !self.heuristics.is_empty() {
            i %= self.heuristics.len();
            let next = self.heuristics[i].improve(instance, current.clone());

            if P::score(&next) > P::score(&current) {
                current = next;
                i += 1;
            } else {
                self.heuristics.remove(i);
            }

            self.termination.iteration();
        }
        current
    }
}

pub struct ExploreOnce<N>(pub N);

impl<P: Problem, N: Neighborhood<P>> ImprovementHeuristic<P> for ExploreOnce<N> {
    fn improve(&mut self, instance: &P, current: P::Solution) -> P::Solution {
        N::neighbors_iter(instance, &current)
            .max_by_key(|s| P::score(s))
            .unwrap_or(current)
    }
}
