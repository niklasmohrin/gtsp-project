use std::iter;

use crate::{
    gtsp::{GtspProblem, Solution},
    Move, MoveNeighborhood, Problem, Ring,
};

pub struct TwoOptNeighborhood;

pub struct TwoOptMove<'p, R> {
    problem: &'p GtspProblem<R>,
    current: &'p Solution<R>,
    i: usize,
    h: usize,
}

impl<'p, R: Ring> Move<GtspProblem<R>> for TwoOptMove<'p, R> {
    fn score_increase(&self) -> <GtspProblem<R> as Problem>::Score {
        let removed_cost = self.current.forward_cost(self.problem, self.i..=self.h + 1);
        let added_cost = self.current.forward_cost(
            self.problem,
            iter::once(self.i)
                .chain((self.i + 1..=self.h).rev())
                .chain(iter::once(self.h + 1)),
        );
        removed_cost - added_cost
    }

    fn is_improving(&self) -> bool {
        self.score_increase() > 0.into()
    }

    fn into_solution(self) -> <GtspProblem<R> as Problem>::Solution {
        let j = self.i + 1;
        let k = self.h + 1;

        let mut tour = self.current.tour().to_owned();
        tour[j..k].reverse();
        Solution::new(self.problem, tour).assert_weight(self.current.weight - self.score_increase())
    }
}

impl<R: Ring> MoveNeighborhood<GtspProblem<R>> for TwoOptNeighborhood {
    type Move<'c> = TwoOptMove<'c , R> where R: 'c;

    // TODO: generate on the fly
    type Iter<'c> = <Vec<Self::Move<'c>> as IntoIterator>::IntoIter where R: 'c;

    fn moves_iter<'c, 'p: 'c>(
        problem: &'p GtspProblem<R>,
        current: &'c <GtspProblem<R> as Problem>::Solution,
    ) -> Self::Iter<'c> {
        let mut moves = Vec::new();

        for i in 0..current.tour().len() - 1 {
            let j = i + 1;
            for h in j + 1..current.tour().len() {
                let k = h + 1;
                if i == 0 && k == current.tour().len() {
                    continue;
                }
                moves.push(TwoOptMove {
                    problem,
                    current,
                    i,
                    h,
                })
            }
        }

        moves.into_iter()
    }
}
