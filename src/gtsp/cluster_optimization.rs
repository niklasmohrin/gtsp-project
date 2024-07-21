use itertools::Itertools as _;

use crate::{
    gtsp::{GtspProblem, Solution},
    ImprovementHeuristic, Ring,
};

pub struct ClusterOptimization;

impl<R: Ring> ImprovementHeuristic<GtspProblem<R>> for ClusterOptimization {
    fn improve(
        &mut self,
        instance: &GtspProblem<R>,
        current: <GtspProblem<R> as crate::Problem>::Solution,
    ) -> Solution<R> {
        let mut cluster_order = vec![0; instance.clusters.len()];
        for (i, c) in instance.clusters.iter().enumerate() {
            cluster_order[current.tour().iter().position(|v| c.contains(v)).unwrap()] = i;
        }

        let starting_cluster = cluster_order
            .iter()
            .enumerate()
            .min_by_key(|(_, &i)| instance.clusters[i].len())
            .unwrap()
            .0;

        (0..instance.clusters[cluster_order[starting_cluster]].len())
            .map(|s_index| {
                let mut parent = vec![None; instance.number_of_vertices];
                let mut current_dist =
                    vec![None; instance.clusters[cluster_order[starting_cluster]].len()];
                current_dist[s_index] = Some(R::from(0));

                for i in 0..instance.clusters.len() {
                    let current_cluster_i = (starting_cluster + i) % instance.clusters.len();
                    let next_cluster_i = (current_cluster_i + 1) % instance.clusters.len();
                    let current_cluster = cluster_order[current_cluster_i];
                    let next_cluster = cluster_order[next_cluster_i];

                    let mut next_dist = vec![None; instance.clusters[next_cluster].len()];

                    let edges = instance.clusters[current_cluster]
                        .iter()
                        .enumerate()
                        .cartesian_product(instance.clusters[next_cluster].iter().enumerate());
                    for ((a_i, &a), (b_i, &b)) in edges {
                        if let Some(a_dist) = current_dist[a_i] {
                            let relaxed_dist = a_dist + instance.dist(a, b);
                            if next_dist[b_i].filter(|&d| d <= relaxed_dist).is_none() {
                                next_dist[b_i] = Some(relaxed_dist);
                                parent[b] = Some(a);
                            }
                        }
                    }

                    current_dist = next_dist;
                }

                let mut tour = vec![instance.clusters[cluster_order[starting_cluster]][s_index]];
                while tour.len() < instance.clusters.len() {
                    tour.push(parent[*tour.last().unwrap()].unwrap());
                }
                tour.reverse();
                Solution::new(instance, tour)
            })
            .min_by_key(|tour| tour.weight())
            .unwrap()
    }
}
