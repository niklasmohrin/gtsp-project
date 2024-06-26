use rand::Rng;

pub mod gtsp;
pub mod localsearch;

pub trait Problem {
    type Score: Ord;
    type Solution;

    fn score(solution: &Self::Solution) -> Self::Score;
    fn make_intial_solution(&self, rng: impl Rng) -> Self::Solution;
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

pub trait MetaHeuristic<P: Problem> {
    fn run(instance: &P, rng: impl Rng) -> P::Solution;
}
