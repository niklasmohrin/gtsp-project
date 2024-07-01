use std::{
    fmt::Debug,
    io::BufRead,
    iter::Sum,
    ops::{Add, Sub},
    str::FromStr,
};

use anyhow::{bail, Context};
use itertools::Itertools;
use rand::{seq::SliceRandom, Rng};

use crate::Problem;

pub mod neighborhoods;

pub trait Ring:
    Debug + Copy + Ord + From<u8> + Add<Output = Self> + Sub<Output = Self> + Sum
{
}
impl<T: Debug + Copy + Ord + From<u8> + Add<Output = Self> + Sub<Output = Self> + Sum> Ring for T {}

pub struct GtspProblem<R> {
    number_of_vertices: usize,
    clusters: Vec<Vec<usize>>,
    is_symmetric: bool,
    is_triangle: bool,
    dist: Vec<Vec<R>>,
}

impl<R> GtspProblem<R>
where
    R: FromStr,
    <R as FromStr>::Err: std::error::Error + Send + Sync + 'static,
{
    pub fn read_from_text(reader: impl BufRead) -> anyhow::Result<Self> {
        let mut lines = reader.lines();
        let number_of_vertices = lines
            .next()
            .context("read line")??
            .strip_prefix("N: ")
            .context("strip prefix")?
            .parse()?;
        let number_of_clusters = lines
            .next()
            .context("read line")??
            .strip_prefix("M: ")
            .context("strip prefix")?
            .parse()?;
        let is_symmetric = match lines
            .next()
            .context("read line")??
            .strip_prefix("Symmetric: ")
            .context("strip prefix")?
        {
            "true" => true,
            "false" => false,
            _ => bail!("invalid boolean value"),
        };
        let is_triangle = match lines
            .next()
            .context("read line")??
            .strip_prefix("Triangle: ")
            .context("strip prefix")?
        {
            "true" => true,
            "false" => false,
            _ => bail!("invalid boolean value"),
        };

        let clusters = (0..number_of_clusters)
            .map(|_| {
                let nums_line = lines.next().context("read line")??;
                let mut nums = nums_line.split_whitespace();
                let cluster_size = nums.next().context("read num")?.parse()?;
                (0..cluster_size)
                    .map(|_| Ok(nums.next().context("read num")?.parse::<usize>()? - 1))
                    .collect::<anyhow::Result<Vec<_>>>()
            })
            .collect::<anyhow::Result<Vec<_>>>()?;

        let dist = (0..number_of_vertices)
            .map(|_| {
                let nums_line = lines.next().context("read line")??;
                let mut nums = nums_line.split_whitespace();
                (0..number_of_vertices)
                    .map(|_| Ok(nums.next().context("read num")?.parse::<R>()?))
                    .collect::<anyhow::Result<Vec<_>>>()
            })
            .collect::<anyhow::Result<Vec<_>>>()?;

        Ok(Self {
            number_of_vertices,
            clusters,
            is_symmetric,
            is_triangle,
            dist,
        })
    }
}

impl<R> GtspProblem<R> {
    pub fn dist(&self, u: usize, v: usize) -> R
    where
        R: Copy,
    {
        debug_assert_ne!(u, v);
        self.dist[u][v]
    }
    pub fn number_of_vertices(&self) -> usize {
        self.number_of_vertices
    }
    pub fn is_symmetric(&self) -> bool {
        self.is_symmetric
    }
}

#[derive(Debug, Clone)]
pub struct Solution<R> {
    weight: R,
    tour: Vec<usize>,
}

impl<R> Solution<R>
where
    R: FromStr,
    <R as FromStr>::Err: std::error::Error + Send + Sync + 'static,
{
    pub fn read_from_text(reader: impl BufRead) -> anyhow::Result<Self> {
        let mut lines = reader.lines();
        let tour_size = lines.next().context("read line")??.parse::<usize>()?;
        let weight = lines.next().context("read line")??.parse()?;
        let tour = (0..tour_size)
            .map(|_| Ok(lines.next().context("read line")??.parse::<usize>()? - 1))
            .collect::<anyhow::Result<Vec<_>>>()?;

        Ok(Self { weight, tour })
    }
}

impl<R: Ring> Solution<R> {
    pub fn new(problem: &GtspProblem<R>, tour: Vec<usize>) -> Self {
        let mut this = Self {
            tour,
            weight: 0.into(),
        };
        this.weight = this.forward_cost(problem, 0..=this.tour.len());
        this
    }
    pub fn weight(&self) -> R {
        self.weight
    }
    pub fn tour(&self) -> &[usize] {
        self.tour.as_ref()
    }

    pub fn assert_weight(self, w: R) -> Self {
        assert_eq!(self.weight, w);
        self
    }

    pub fn arc_cost(&self, problem: &GtspProblem<R>, i: usize, j: usize) -> R {
        problem.dist(
            self.tour[i % self.tour.len()],
            self.tour[j % self.tour.len()],
        )
    }

    pub fn forward_cost(
        &self,
        problem: &GtspProblem<R>,
        range: impl IntoIterator<Item = usize>,
    ) -> R {
        range
            .into_iter()
            .tuple_windows()
            .map(|(i, j)| self.arc_cost(problem, i, j))
            .sum()
    }
}

impl<R: Ring> Problem for GtspProblem<R> {
    type Score = R;

    type Solution = Solution<R>;

    fn score(solution: &Self::Solution) -> Self::Score {
        solution.weight()
    }

    fn make_intial_solution(&self, mut rng: impl Rng) -> Self::Solution {
        Solution::new(
            self,
            self.clusters
                .iter()
                .map(|c| *c.choose(&mut rng).expect("cluster was empty"))
                .collect(),
        )
    }
}
