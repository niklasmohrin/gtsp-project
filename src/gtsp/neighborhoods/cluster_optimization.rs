use itertools::Itertools as _;

use crate::{
    gtsp::{GtspProblem, Ring, Solution},
    Neighborhood,
};

pub struct ClusterOptimization;

// TODO: maybe only implement metaheuristic
impl<R: Ring> Neighborhood<GtspProblem<R>> for ClusterOptimization {
    type Iter = <Vec<Solution<R>> as IntoIterator>::IntoIter;

    fn neighbors_iter<'c, 'p: 'c>(
        problem: &'p GtspProblem<R>,
        current: &'c Solution<R>,
    ) -> Self::Iter {
        let mut cluster_order = vec![0; problem.clusters.len()];
        for (i, c) in problem.clusters.iter().enumerate() {
            cluster_order[current.tour().iter().position(|v| c.contains(v)).unwrap()] = i;
        }

        let starting_cluster = cluster_order
            .iter()
            .enumerate()
            .min_by_key(|(_, &i)| problem.clusters[i].len())
            .unwrap()
            .0;

        let mut solutions = Vec::new();

        for s_index in 0..problem.clusters[cluster_order[starting_cluster]].len() {
            let mut parent = vec![None; problem.number_of_vertices];
            let mut current_dist = vec![None; problem.clusters[cluster_order[starting_cluster]].len()];
            current_dist[s_index] = Some(R::from(0));

            for i in 0..problem.clusters.len() {
                let current_cluster_i = (starting_cluster + i) % problem.clusters.len();
                let next_cluster_i = (current_cluster_i + 1) % problem.clusters.len();
                let current_cluster = cluster_order[current_cluster_i];
                let next_cluster = cluster_order[next_cluster_i];

                let mut next_dist = vec![None; problem.clusters[next_cluster].len()];

                let edges = problem.clusters[current_cluster]
                    .iter()
                    .enumerate()
                    .cartesian_product(problem.clusters[next_cluster].iter().enumerate());
                for ((a_i, &a), (b_i, &b)) in edges {
                    if let Some(a_dist) = current_dist[a_i] {
                        let relaxed_dist = a_dist + problem.dist(a, b);
                        if next_dist[b_i].filter(|&d| d <= relaxed_dist).is_none() {
                            next_dist[b_i] = Some(relaxed_dist);
                            parent[b] = Some(a);
                        }
                    }
                }

                current_dist = next_dist;
            }

            let mut tour = vec![problem.clusters[cluster_order[starting_cluster]][s_index]];
            while tour.len() < problem.clusters.len() {
                tour.push(parent[*tour.last().unwrap()].unwrap());
            }
            tour.reverse();
            solutions.push(Solution::new(problem, tour));
        }

        solutions.into_iter()
    }
}
