use std::time::Instant;

use crate::tsp::neighborhoods::transition::Transition;
use crate::tsp::neighborhoods::neighborhood::Neighborhood;
use crate::tsp::def::TSPSolution;
use crate::tsp::def::TSPInstance;
use crate::traits::{Solver, Instance};
use std::cmp::Ordering::Equal;
use std::cmp::Ordering;


pub struct CandidateSteepestSolver {
    initial_solver: Box<dyn Solver<TSPInstance, TSPSolution>>,
    transition: fn() -> Vec<Box<dyn Transition>>,
}

impl CandidateSteepestSolver {
    pub fn new(initial_solver: Box<dyn Solver<TSPInstance, TSPSolution>>, transition: fn() -> Vec<Box<dyn Transition>>) -> CandidateSteepestSolver {
        CandidateSteepestSolver {
            initial_solver,
            transition,
        }
    }
}

impl Solver<TSPInstance, TSPSolution> for CandidateSteepestSolver {
    fn solve(&self, start_vertex: usize, instance: &TSPInstance) -> TSPSolution {
        let mut solution = self.initial_solver.solve(start_vertex, instance);
        let mut best_10: Vec<Vec<usize>> = Vec::new();
        for i in 0..instance.dimension {
            let distances: Vec<f32> = instance.distance_cache[i].clone();
            let indices: Vec<usize> = (0..instance.dimension).collect();
            let mut dist_i_vector: Vec<(f32, usize)> = distances.into_iter().zip(indices.into_iter()).collect();
            dist_i_vector.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
            best_10.push(dist_i_vector.iter().take(10).map(|x| x.1).collect());
        }
        let mut neighborhood = Neighborhood::new((self.transition)(), &solution, false);
        let mut improvement_flag = true;
        while improvement_flag {
            improvement_flag = false;
            neighborhood.reset();
            let mut best_move: (f32, usize) = (1.0, 0);
            let mut i = 0;
            loop {
                // let state =
                let score = neighborhood.score(i, &instance, &solution);
                match score {
                    Some(x) => {
                        if x.0 < best_move.0 {
                            best_move = x
                        }
                    }
                    None => break
                }
                i += 1;
            }
            if best_move.0 < 0.0 {
                neighborhood.apply_transition(best_move.1, &mut solution);
                improvement_flag = true;
            }
        }
        solution
    }
}