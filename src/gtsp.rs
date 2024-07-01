use std::{
    io::BufRead,
    iter::{self, Sum},
    ops::{Add, Sub},
    str::FromStr,
};

use anyhow::{bail, Context};
use itertools::Itertools;
use rand::{seq::SliceRandom, Rng};

use crate::{Move, MoveNeighborhood, Problem};

pub trait Ring: Copy + Ord + From<u8> + Add<Output = Self> + Sub<Output = Self> + Sum {}
impl<T: Copy + Ord + From<u8> + Add<Output = Self> + Sub<Output = Self> + Sum> Ring for T {}

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
        self.dist[u][v]
    }
    pub fn number_of_vertices(&self) -> usize {
        self.number_of_vertices
    }
    pub fn is_symmetric(&self) -> bool {
        self.is_symmetric
    }
}

#[derive(Debug)]
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
        let weight = tour
            .windows(2)
            .map(|win| problem.dist(win[0], win[1]))
            .sum::<R>()
            + problem.dist(tour[tour.len() - 1], tour[0]);

        Self { weight, tour }
    }
    pub fn weight(&self) -> &R {
        &self.weight
    }
    pub fn tour(&self) -> &[usize] {
        self.tour.as_ref()
    }

    pub fn forward_cost(&self, problem: &GtspProblem<R>, range: impl Iterator<Item = usize>) -> R {
        range
            .tuple_windows()
            .map(|(a, b)| {
                problem.dist(
                    self.tour[a % self.tour.len()],
                    self.tour[b % self.tour.len()],
                )
            })
            .sum()
    }
}

impl<R: Ring> Problem for GtspProblem<R> {
    type Score = R;

    type Solution = Solution<R>;

    fn score(solution: &Self::Solution) -> Self::Score {
        *solution.weight()
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

    fn into_solution(&self) -> <GtspProblem<R> as Problem>::Solution {
        let j = self.i + 1;
        let k = self.h + 1;

        let mut tour = self.current.tour().to_owned();
        tour[j..k].reverse();
        Solution::new(self.problem, tour)
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
