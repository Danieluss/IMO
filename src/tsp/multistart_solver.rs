use crate::tsp::def::{TSPSolution, TSPInstance};
use crate::traits::{Solver, Instance};

pub struct MultiStartSolver {
    sub_solver: Box<dyn Solver<TSPInstance, TSPSolution>>,
    no_iterations: usize
}


impl MultiStartSolver {
    pub fn new(sub_solver: Box<dyn Solver<TSPInstance, TSPSolution>>, no_iterations: usize) -> MultiStartSolver {
        MultiStartSolver {
            sub_solver,
            no_iterations
        }
    }
}

impl Solver<TSPInstance, TSPSolution> for MultiStartSolver {
    fn solve(&self, start_vertex: usize, instance: &TSPInstance) -> TSPSolution {
        let mut solution = self.sub_solver.solve(start_vertex, instance);
        let mut best_solution: (f32, TSPSolution) = (instance.eval(&solution), solution);
        for i in 0..self.no_iterations - 1 {
            let mut it_solution = self.sub_solver.solve(start_vertex, instance);
            let mut it_score = instance.eval(&it_solution);
            if it_score < best_solution.0 {
                best_solution = (it_score, it_solution)
            }
            println!("{} {}", i, best_solution.0)
        }
        best_solution.1
    }

    fn solve_s(&self, start_vertex: usize, instance: &TSPInstance, solution: TSPSolution) -> TSPSolution {
        unimplemented!()
    }
}