use crate::{
    gtsp::{GtspProblem, Solution},
    Neighborhood, Ring,
};

pub struct InsertsNeighborhood;

impl<R: Ring> Neighborhood<GtspProblem<R>> for InsertsNeighborhood {
    type Iter = <Vec<Solution<R>> as IntoIterator>::IntoIter;

    fn neighbors_iter<'c, 'p: 'c>(
        problem: &'p GtspProblem<R>,
        current: &'c <GtspProblem<R> as crate::Problem>::Solution,
    ) -> Self::Iter {
        let n = current.tour().len();
        let mut solutions = Vec::new();
        for i in 0..n {
            let cluster = problem
                .clusters
                .iter()
                .position(|c| c.contains(&current.tour()[i]))
                .unwrap();
            for j in 0..n {
                if i == j || i == (j + 1) % n || j == (i + 1) % n {
                    continue;
                }
                for &chosen_vertex in problem.clusters[cluster].iter() {
                    let mut tour = current.tour().to_owned();
                    tour.remove(i);
                    tour.insert(j, chosen_vertex);
                    solutions.push(Solution::new(problem, tour));
                }
            }
        }
        solutions.into_iter()
    }
}
