use std::time::Instant;

use rand::{Rng, thread_rng};
use rand::seq::SliceRandom;

use crate::traits::{Instance, Solver};
use crate::tsp::def::{TSPInstance, TSPSolution};
use crate::tsp::neighborhoods::neighborhood::Neighborhood;
use crate::tsp::neighborhoods::transition::Transition;
use crate::tsp::solver::GreedySolver;
use crate::tsp::pickers::regret_picker::RegretPicker;
use std::collections::{BinaryHeap, BTreeSet, HashMap};
use std::collections::HashSet;
use std::cmp::{Ordering, max};
use crate::utils::{random_combination, MinFloat};
use crate::tsp::similarity::Similarity;

pub struct ConvexityChecker {
    best_solver: Box<dyn Solver<TSPInstance, TSPSolution>>,
    solver: Box<dyn Solver<TSPInstance, TSPSolution>>,
    no_solutions: usize,
    similarity: Box<dyn Similarity>,
}

impl ConvexityChecker {
    pub fn new(best_solver: Box<dyn Solver<TSPInstance, TSPSolution>>,
               solver: Box<dyn Solver<TSPInstance, TSPSolution>>,
               no_solutions: usize,
               similarity: Box<dyn Similarity>) -> ConvexityChecker {
        ConvexityChecker {
            best_solver,
            solver,
            no_solutions,
            similarity,
        }
    }
}


impl Solver<TSPInstance, TSPSolution> for ConvexityChecker {
    fn solve(&self, _: usize, instance: &TSPInstance) -> TSPSolution {
        println!(">>> ConvexityCheck");
        let mut vec = vec![];
        let EMPTY = TSPSolution {
            perm_a: vec![],
            perm_b: vec![],
            cycle: vec![],
            order: vec![],
        };
        let mut best_solution = (f32::INFINITY, 0);
        for i in 0..self.no_solutions {
            let solution = self.solver.solve(i % instance.dimension, instance);
            let score = instance.eval(&solution);
            if score < best_solution.0 {
                best_solution = (score, i);
            }
            vec.push(solution);
        }
        let mut best_solution: (f32, &TSPSolution, usize) = (
            best_solution.0,
            &vec[best_solution.1],
            best_solution.1);

        let solution = self.best_solver.solve(0, instance);
        if instance.eval(&solution) < best_solution.0 {
            best_solution = (instance.eval(&solution), &solution, 0);
        }

        let mut avg_sim_vec = vec![];
        for solution_i in 0..self.no_solutions {
            let solution = &vec[solution_i];
            let mut avg_sim = 0.;
            for solution_j in 0..self.no_solutions {
                let similarity = self.similarity.sim(instance, solution, &vec[solution_j]) as f64;
                if solution_i != solution_j {
                    avg_sim += similarity;
                }
            }
            avg_sim /= self.no_solutions as f64;
            avg_sim_vec.push(avg_sim);
        }
        println!("score, sim_to_best, avg_sim");
        for solution_i in 0..self.no_solutions {
            let solution = &vec[solution_i];
            println!("{}, {:.3}, {:.3}",
                     instance.eval(solution),
                     self.similarity.sim(instance, solution, &best_solution.1) as f64 / instance.dimension as f64,
                     avg_sim_vec[solution_i] / instance.dimension as f64
            );
        }

        println!("<<< ConvexityCheck");
        vec.remove(best_solution.2)
    }

    fn solve_s(&self, start_vertex: usize, instance: &TSPInstance, solution: TSPSolution) -> TSPSolution {
        unimplemented!()
    }
}
