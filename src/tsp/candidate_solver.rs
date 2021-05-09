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
            }
            nearest_vertices.push(u);
        }
        
        while improvement_flag {
            improvement_flag = false;
            let mut min_score: f32 = 0.0;
            let mut best_pair: (usize, usize) = (0, 0);
            for i in 0..instance.dimension {
                for k in 0..nearest_vertices[i].len() {
                    let j = nearest_vertices[i][k];
                    if solution.cycle[i] == solution.cycle[j] && (solution.order[i] as isize -solution.order[j] as isize).abs() <= 1 {
                        continue;
                    }
                    let score: Option<f32>;
                    if solution.cycle[i] != solution.cycle[j] {
                        let j_prev;
                        let j_next;
                        let vertex_prev;
                        let vertex_next;
                        let score_prev;
                        let score_next;
                        if solution.cycle[i] == 0 {
                            j_prev = (solution.order[j]+solution.perm_b.len()-1)%solution.perm_b.len();
                            j_next = (solution.order[j]+1)%solution.perm_b.len();
                            vertex_prev = solution.perm_b[j_prev];
                            vertex_next = solution.perm_b[j_next];
                            score_prev = inter_cycle_transition.score_explicit(solution.order[i], j_prev, instance, &solution);
                            score_next = inter_cycle_transition.score_explicit(solution.order[i], j_next, instance, &solution);
                        } else {
                            j_prev = (solution.order[j]+solution.perm_a.len()-1)%solution.perm_a.len();
                            j_next = (solution.order[j]+1)%solution.perm_a.len();
                            vertex_prev = solution.perm_a[j_prev];
                            vertex_next = solution.perm_a[j_next];
                            score_prev = inter_cycle_transition.score_explicit(j_prev, solution.order[i], instance, &solution);
                            score_next = inter_cycle_transition.score_explicit(j_next, solution.order[i], instance, &solution);
                        }
                        if score_prev.unwrap() < min_score {
                            min_score = score_prev.unwrap();
                            best_pair = (i, vertex_prev);
                        }
                        if score_next.unwrap() < min_score {
                            min_score = score_next.unwrap();
                            best_pair = (i, vertex_next);
                        }
                    } else {
                        let i_prev;
                        let j_prev;
                        if solution.cycle[i] == 0 {
                            i_prev = (solution.order[i]+solution.perm_a.len()-1)%solution.perm_a.len();
                            j_prev = (solution.order[j]+solution.perm_a.len()-1)%solution.perm_a.len();
                        } else {
                            i_prev = (solution.order[i]+solution.perm_b.len()-1)%solution.perm_b.len();
                            j_prev = (solution.order[j]+solution.perm_b.len()-1)%solution.perm_b.len();
                        }
                        score = edges_transition.score_explicit(solution.cycle[i], solution.order[i], solution.order[j], instance, &solution);
                        match score {
                            Some(x) => {
                                if score.unwrap() < min_score {
                                    min_score = score.unwrap();
                                    best_pair = (i, j);
                                }
                            }
                            None => {}
                        }
                        let nscore = edges_transition.score_explicit(solution.cycle[i], solution.order[i_prev], solution.order[j_prev], instance, &solution);
                        match nscore {
                            Some(x) => {
                                if score.unwrap() < min_score {
                                    min_score = score.unwrap();
                                    best_pair = (i_prev, j_prev);
                                }
                            }
                            None => {}
                        }
                    }    
                }
            }
            if min_score < 0.0 {
                improvement_flag = true;
                let (i, j) = best_pair;
                // println!("{} {} {} {}", solution.cycle[i], solution.order[i], solution.cycle[j], solution.order[j]);
                // let s1 = instance.eval(&solution);
                if solution.cycle[i] != solution.cycle[j] {
                    if solution.cycle[i] == 0 {
                        inter_cycle_transition.apply_explicit(solution.order[i], solution.order[j], &mut solution)
                    } else {
                        inter_cycle_transition.apply_explicit(solution.order[j], solution.order[i], &mut solution)
                    }
                } else {
                    edges_transition.apply_explicit(solution.cycle[i], solution.order[i], solution.order[j], &mut solution)
                }
                // for i in 0..instance.dimension {
                //     if solution.cycle[i] == 0 {
                //         assert_eq!(i, solution.perm_a[solution.order[i]]);
                //     } else {
                //         assert_eq!(i, solution.perm_b[solution.order[i]]);
                //     }
                // }
                // let s2 = instance.eval(&solution);
                // assert_eq!((s2-s1) as isize, min_score as isize);
            }
        }
        solution
    }

    fn solve_s(&self, start_vertex: usize, instance: &TSPInstance, solution: TSPSolution) -> TSPSolution {
        unimplemented!()
    }
}

