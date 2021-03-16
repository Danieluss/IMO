use std::time::Instant;

use crate::tsp::neighborhoods::transition::Transition;
use crate::tsp::neighborhoods::neighborhood::Neighborhood;
use crate::tsp::def::TSPSolution;
use crate::tsp::def::TSPInstance;
use crate::traits::{Solver, Instance};

pub struct LocalGreedySolver {
    initial_solver: Box<dyn Solver<TSPInstance, TSPSolution>>,
    transition: fn() -> Vec<Box<dyn Transition>>
}

impl LocalGreedySolver {
    pub fn new(initial_solver: Box<dyn Solver<TSPInstance, TSPSolution>>, transition: fn() -> Vec<Box<dyn Transition>>) -> LocalGreedySolver {
        LocalGreedySolver {
            initial_solver,
            transition
        }
    }
}

impl Solver<TSPInstance, TSPSolution> for LocalGreedySolver {
    fn solve(&self, start_vertex: usize, instance: &TSPInstance) -> TSPSolution {
        let mut solution = self.initial_solver.solve(start_vertex, instance);
        let mut neighborhood = Neighborhood::new((self.transition)(), &solution, true);
        let mut improvement_flag = true;
        while improvement_flag {
            improvement_flag = false;
            neighborhood.reset();
            loop {
                let score = neighborhood.next(instance, &solution);
                match score {
                    Some(x) => {
                        if x.0 < 0.0 {
                            improvement_flag = true;
                            // let a = instance.eval(&solution);
                            neighborhood.apply_transition(x.1, &mut solution);
                            // let b = instance.eval(&solution);
                            // if (a+x.0-b).abs() > 0.1 {
                            //     println!("{}{}={}", a, x.0, b);
                            //     panic!("Wrong delta");
                            // }
                            break
                        }
                    },
                    None => break
                }
            }
        }
        solution
    }
}

pub struct LocalSteepestSolver {
    initial_solver: Box<dyn Solver<TSPInstance, TSPSolution>>,
    transition: fn() -> Vec<Box<dyn Transition>>
}

impl LocalSteepestSolver {
    pub fn new(initial_solver: Box<dyn Solver<TSPInstance, TSPSolution>>, transition: fn() -> Vec<Box<dyn Transition>>) -> LocalSteepestSolver {
        LocalSteepestSolver {
            initial_solver,
            transition
        }
    }
}

impl Solver<TSPInstance, TSPSolution> for LocalSteepestSolver {
    fn solve(&self, start_vertex: usize, instance: &TSPInstance) -> TSPSolution {
        let mut solution = self.initial_solver.solve(start_vertex, instance);
        let mut neighborhood = Neighborhood::new((self.transition)(), &solution, false);
        let mut improvement_flag = true;
        while improvement_flag {
            improvement_flag = false;
            neighborhood.reset();
            let mut best_move: (f32, usize) = (1.0, 0);
            loop {
                let score = neighborhood.next(instance, &solution);
                match score {
                    Some(x) => {
                        if x.0 < best_move.0 {
                            best_move = x
                        }
                    },
                    None => break
                }
            }
            if best_move.0 < 0.0 {
                neighborhood.apply_transition(best_move.1, &mut solution);
                improvement_flag = true;
            }
        }
        solution
    }
}

pub struct LocalRandomWalker {
    initial_solver: Box<dyn Solver<TSPInstance, TSPSolution>>,
    transition: fn() -> Vec<Box<dyn Transition>>
}

impl LocalRandomWalker {
    pub fn new(initial_solver: Box<dyn Solver<TSPInstance, TSPSolution>>, transition: fn() -> Vec<Box<dyn Transition>>) -> LocalRandomWalker {
        LocalRandomWalker {
            initial_solver,
            transition
        }
    }
}

impl Solver<TSPInstance, TSPSolution> for LocalRandomWalker {
    fn solve(&self, start_vertex: usize, instance: &TSPInstance) -> TSPSolution {
        let start = Instant::now();
        let mut solution = self.initial_solver.solve(start_vertex, instance);
        let mut neighborhood = Neighborhood::new((self.transition)(), &solution, true);
        let mut current_score = instance.eval(&solution);
        let mut best_solution = (current_score, solution.clone());
        while  start.elapsed().as_secs_f32() < 1.0 {
            neighborhood.reset();
            loop {
                let score = neighborhood.next(instance, &solution);
                match score {
                    Some(x) => {
                        current_score+=x.0;
                        // let a = instance.eval(&solution);
                        neighborhood.apply_transition(x.1, &mut solution);
                        if current_score < best_solution.0 {
                            best_solution = (current_score, solution.clone());
                        }
                        // let b = instance.eval(&solution);
                        // if (a+x.0-b).abs() > 0.1 {
                        //     println!("{}{}={}", a, x.0, b);
                        //     panic!("Wrong delta");
                        // }
                        break
                    },
                    None => break
                }
            }
        }
        best_solution.1
    }
}