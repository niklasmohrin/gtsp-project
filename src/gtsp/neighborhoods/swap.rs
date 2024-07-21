use crate::{
    gtsp::{GtspProblem, Solution},
    Move, MoveNeighborhood, Problem, Ring,
};

pub struct SwapNeighborhood;

pub struct SwapMove<'p, R> {
    current: &'p Solution<R>,
    new: Solution<R>,
}

impl<'p, R: Ring> SwapMove<'p, R> {
    pub fn new(problem: &GtspProblem<R>, current: &'p Solution<R>, i: usize, j: usize) -> Self {
        let mut tour = current.tour.to_owned();
        tour.swap(i, j);
        Self {
            current,
            new: Solution::new(problem, tour),
        }
    }
}

impl<'p, R: Ring> Move<GtspProblem<R>> for SwapMove<'p, R> {
    fn score_increase(&self) -> <GtspProblem<R> as Problem>::Score {
        self.current.weight() - self.new.weight()
    }

    fn is_improving(&self) -> bool {
        self.score_increase() > 0.into()
    }

    fn into_solution(&self) -> <GtspProblem<R> as Problem>::Solution {
        self.new.clone()
    }
}

impl<R: Ring> MoveNeighborhood<GtspProblem<R>> for SwapNeighborhood {
    type Move<'c> = SwapMove<'c , R> where R: 'c;

    // TODO: generate on the fly
    type Iter<'c> = <Vec<Self::Move<'c>> as IntoIterator>::IntoIter where R: 'c;

    fn moves_iter<'c, 'p: 'c>(
        problem: &'p GtspProblem<R>,
        current: &'c <GtspProblem<R> as Problem>::Solution,
    ) -> Self::Iter<'c> {
        let mut moves = Vec::new();

        for i in 0..current.tour().len() {
            for j in i + 1..current.tour().len() {
                moves.push(SwapMove::new(problem, current, i, j));
            }
        }

        moves.into_iter()
    }
}
