use itertools::Itertools;
use std::{
    fmt::Debug,
    iter::Sum,
    ops::{Add, Neg, Sub},
};

pub mod chain;
pub mod cycle_neighborhoods;
pub mod gtsp;
pub mod localsearch;
pub mod multistart;
pub mod tabusearch;
pub mod termination;

pub trait Ring:
    Debug + Copy + Ord + From<u8> + Add<Output = Self> + Sub<Output = Self> + Neg<Output = Self> + Sum
{
}
impl<
        T: Debug
            + Copy
            + Ord
            + From<u8>
            + Add<Output = Self>
            + Sub<Output = Self>
            + Neg<Output = Self>
            + Sum,
    > Ring for T
{
}

pub trait Problem {
    type Score: Ord;
    type Solution;

    fn score(solution: &Self::Solution) -> Self::Score;
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

pub trait InitialSolution<P: Problem> {
    fn make_intial_solution(&mut self, instance: &P) -> P::Solution;
}

pub trait ImprovementHeuristic<P: Problem> {
    fn improve(&mut self, instance: &P, current: P::Solution) -> P::Solution;
}

pub trait MetaHeuristic<P: Problem> {
    fn run(self, instance: &P) -> P::Solution;
}

pub struct ImproveInitial<In, Im> {
    initial: In,
    improvement: Im,
}

impl<In, Im> ImproveInitial<In, Im> {
    pub fn new(initial: In, improvement: Im) -> Self {
        Self {
            initial,
            improvement,
        }
    }
}

impl<P, In, Im> MetaHeuristic<P> for ImproveInitial<In, Im>
where
    P: Problem,
    In: InitialSolution<P>,
    Im: ImprovementHeuristic<P>,
{
    fn run(mut self, instance: &P) -> <P as Problem>::Solution {
        self.improvement
            .improve(instance, self.initial.make_intial_solution(instance))
    }
}
