use crate::tsp::similarity::Similarity;
use crate::tsp::def::{TSPSolution, TSPInstance};

pub struct VertexSimilarity;

impl VertexSimilarity {
    pub fn new() -> VertexSimilarity {
        VertexSimilarity {}
    }
}

impl Similarity for VertexSimilarity {
    fn sim(&self, instance: &TSPInstance, solution_a: &TSPSolution, solution_b: &TSPSolution) -> usize {
        let mut similarity = 0;

        for vertex_i in 0..instance.dimension {
            if solution_a.cycle[vertex_i] == solution_b.cycle[vertex_i] {
                similarity += 1;
            }
        }

        if similarity < instance.dimension / 2 {
            similarity = instance.dimension - similarity;
        }

        return similarity;
    }
}