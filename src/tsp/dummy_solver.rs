use crate::tsp::def::{TSPSolution, TSPInstance};
use crate::traits::{Solver, Instance};

pub struct DummySolver {
}


impl DummySolver {
    pub fn new() -> DummySolver {
        DummySolver {
        }
    }
}

impl Solver<TSPInstance, TSPSolution> for DummySolver {
    fn solve(&self, start_vertex: usize, instance: &TSPInstance) -> TSPSolution {
        panic!("I should've never been called")
    }
}