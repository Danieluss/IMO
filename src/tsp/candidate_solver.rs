use std::time::Instant;
use std::cmp;

use crate::tsp::neighborhoods::transition::Transition;
use crate::tsp::neighborhoods::inter_cycle_transition::InterCycleTransition;
use crate::tsp::neighborhoods::edges_transition::EdgesTransition;
use crate::tsp::def::TSPSolution;
use crate::tsp::def::TSPInstance;
use crate::traits::{Solver, Instance};

pub struct CandidateSolver {
    num_neighbors: usize,
    initial_solver: Box<dyn Solver<TSPInstance, TSPSolution>>,
}

impl CandidateSolver {
    pub fn new(num_neighbors: usize, initial_solver: Box<dyn Solver<TSPInstance, TSPSolution>>) -> CandidateSolver {
        CandidateSolver {
            num_neighbors,
            initial_solver
        }
    }
}

impl Solver<TSPInstance, TSPSolution> for CandidateSolver {
    fn solve(&self, start_vertex: usize, instance: &TSPInstance) -> TSPSolution {
        let mut solution = self.initial_solver.solve(start_vertex, instance);
        let inter_cycle_transition = InterCycleTransition::new();
        let edges_transition = EdgesTransition::new();
        let mut improvement_flag = true;
        let mut nearest_vertices: Vec<Vec<usize>> = Vec::new();
        for i in 0..instance.dimension {
            let mut v: Vec<(f32, usize)> = Vec::new();
            for j in 0..instance.dimension {
                if i == j {
                    continue;
                }
                v.push((instance.dist_k(i, j), j));
            }
            v.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
            let mut u: Vec<usize> = Vec::new();
            for j in 0..cmp::min(instance.dimension, self.num_neighbors) {
                u.push(v[j].1);
                // print!("{}, ", v[j].0);
            }
            // println!("");
            nearest_vertices.push(u);
        }
        // println!("{:?}", nearest_vertices);
        
        while improvement_flag {
            improvement_flag = false;
            let mut min_score: f32 = 0.0;
            let mut best_pair: (usize, usize) = (0, 0);
            for i in 0..instance.dimension {
                for k in 0..nearest_vertices[i].len() {
                    let j = nearest_vertices[i][k];
                    let score: Option<f32>;
                    if solution.cycle[i] != solution.cycle[j] {
                        score = inter_cycle_transition.score_explicit(solution.order[i], solution.order[j], instance, &solution)
                    } else {
                        score = edges_transition.score_explicit(solution.cycle[i], solution.order[i], solution.order[j], instance, &solution)
                    }
                    match score {
                        Some(x) => {
                            if x < min_score {
                                min_score = x;
                                best_pair = (i, j);
                            }
                        }
                        None => {}
                    }
                    
                }
            }
            if min_score < 0.0 {
                improvement_flag = true;
                let (i, j) = best_pair;
                if solution.cycle[i] != solution.cycle[j] {
                    inter_cycle_transition.apply_explicit(solution.order[i], solution.order[j], &mut solution)
                } else {
                    edges_transition.apply_explicit(solution.cycle[i], solution.order[i], solution.order[j], &mut solution)
                }
            }
        }
        solution
    }
}

