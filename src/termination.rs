use std::time::{Duration, Instant};

#[derive(Debug, Clone, Copy)]
enum TerminationKind {
    Iterations(usize),
    Timeout(Instant),
    Never,
}

#[derive(Debug, Clone)]
pub struct Termination {
    kind: TerminationKind,
    iterations: usize,
}

impl Termination {
    pub fn after_iterations(n: usize) -> Self {
        Self {
            kind: TerminationKind::Iterations(n),
            iterations: 0,
        }
    }

    pub fn after_duration(d: Duration) -> Self {
        Self {
            kind: TerminationKind::Timeout(Instant::now() + d),
            iterations: 0,
        }
    }

    pub fn never() -> Self {
        Self {
            kind: TerminationKind::Never,
            iterations: 0,
        }
    }

    pub fn should_terminate(&self) -> bool {
        match self.kind {
            TerminationKind::Iterations(n) => self.iterations >= n,
            TerminationKind::Timeout(instant) => instant < Instant::now(),
            TerminationKind::Never => false,
        }
    }

    pub fn iteration(&mut self) {
        self.iterations += 1;
    }
}

#[cfg(feature = "count-iterations")]
impl Drop for Termination {
    fn drop(&mut self) {
        dbg!(self.iterations);
    }
}
