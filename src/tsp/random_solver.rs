use crate::tsp::def::TSPSolution;
use crate::tsp::def::TSPInstance;
use crate::traits::Solver;
use crate::utils::random_permutation;

pub struct RandomSolver;

impl RandomSolver {
    pub fn new() -> RandomSolver {
        RandomSolver
    }
}

impl Solver<TSPInstance, TSPSolution> for RandomSolver {
    fn solve(&self, start_vertex: usize, instance: &TSPInstance) -> TSPSolution {
        let perm = random_permutation(instance.dimension);
        TSPSolution::new(
            perm[..(perm.len() + 1)/2].to_vec(),
            perm[(perm.len() + 1)/2..].to_vec()
        )
    }
}