use itertools::Itertools;
use rand::Rng;

pub mod gtsp;
pub mod localsearch;
pub mod tabusearch;
pub mod termination;

pub trait Problem {
    type Score: Ord;
    type Solution;

    fn score(solution: &Self::Solution) -> Self::Score;
    fn make_intial_solution(&self, rng: impl Rng) -> Self::Solution;
}

pub trait Neighborhood<P: Problem> {
    type Iter: Iterator<Item = P::Solution>;

    fn neighbors_iter<'c, 'p: 'c>(problem: &'p P, current: &'c P::Solution) -> Self::Iter;
}

pub trait Move<P: Problem> {
    fn score_increase(&self) -> P::Score;
    fn is_improving(&self) -> bool;
    fn into_solution(&self) -> P::Solution;
}

pub trait MoveNeighborhood<P: Problem> {
    type Move<'c>: Move<P> + 'c
    where
        P: 'c;

    type Iter<'c>: Iterator<Item = Self::Move<'c>>
    where
        P: 'c;

    fn moves_iter<'c, 'p: 'c>(problem: &'p P, current: &'c P::Solution) -> Self::Iter<'c>;
}

impl<P: Problem, N: MoveNeighborhood<P>> Neighborhood<P> for N {
    type Iter = <Vec<P::Solution> as IntoIterator>::IntoIter;

    fn neighbors_iter<'c, 'p: 'c>(
        problem: &'p P,
        current: &'c <P as Problem>::Solution,
    ) -> Self::Iter {
        N::moves_iter(problem, current)
            .map(|m| m.into_solution())
            .collect_vec()
            .into_iter()
    }
}

pub trait MetaHeuristic<P: Problem> {
    fn run(self, instance: &P, rng: impl Rng) -> P::Solution;
}
