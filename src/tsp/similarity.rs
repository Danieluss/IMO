use crate::tsp::def::{TSPSolution, TSPInstance};

pub trait Similarity {
    fn sim(&self, instance: &TSPInstance, solution_a: &TSPSolution, solution_b: &TSPSolution) -> usize;
}