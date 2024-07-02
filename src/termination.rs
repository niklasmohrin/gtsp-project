use std::time::Instant;

#[derive(Debug, Clone, Copy)]
pub enum Termination {
    Iterations(usize),
    Timeout(Instant),
}

impl Termination {
    pub fn should_terminate(&self) -> bool {
        match self {
            Self::Iterations(n) => *n == 0,
            Self::Timeout(instant) => *instant < Instant::now(),
        }
    }

    pub fn iteration(&mut self) {
        match *self {
            Self::Iterations(n) => *self = Self::Iterations(n - 1),
            Self::Timeout(_) => {}
        }
    }
}
