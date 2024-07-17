use crate::{ImprovementHeuristic, Problem};

pub struct Chain<I1, I2> {
    i1: I1,
    i2: I2,
}

impl<I1, I2> Chain<I1, I2> {
    pub fn new(i1: I1, i2: I2) -> Self {
        Self { i1, i2 }
    }
}

impl<P, I1, I2> ImprovementHeuristic<P> for Chain<I1, I2>
where
    P: Problem,
    I1: ImprovementHeuristic<P>,
    I2: ImprovementHeuristic<P>,
{
    fn improve(
        &mut self,
        instance: &P,
        current: <P as Problem>::Solution,
    ) -> <P as Problem>::Solution {
        self.i2
            .improve(instance, self.i1.improve(instance, current))
    }
}
