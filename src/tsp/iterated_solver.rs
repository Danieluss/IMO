use crate::tsp::def::{TSPSolution, TSPInstance};
use crate::traits::{Solver, Instance};
use std::time::Instant;
use crate::tsp::neighborhoods::neighborhood::Neighborhood;
use crate::tsp::neighborhoods::transition::Transition;
use rand::seq::SliceRandom;
use rand::{thread_rng, Rng};

pub struct IteratedSolver {
    initial_solver: Box<dyn Solver<TSPInstance, TSPSolution>>,
    sub_solver: Box<dyn Solver<TSPInstance, TSPSolution>>,
    time: f32,
    perturb_min: f32,
    perturb_max: f32,
    transition: fn() -> Vec<Box<dyn Transition>>,
}


impl IteratedSolver {
    pub fn new(initial_solver: Box<dyn Solver<TSPInstance, TSPSolution>>,
               sub_solver: Box<dyn Solver<TSPInstance, TSPSolution>>,
               time: f32,
               perturb_min: f32,
               perturb_max: f32,
               transition: fn() -> Vec<Box<dyn Transition>>) -> IteratedSolver {
        IteratedSolver {
            initial_solver,
            sub_solver,
            time,
            perturb_min,
            perturb_max,
            transition,
        }
    }

    fn perturb(&self, instance: &TSPInstance, mut solution: TSPSolution, neighborhood: &mut Neighborhood) -> TSPSolution {
        let mut i: f32 = 0.0;
        let perturb_size = (instance.dimension as f32) * thread_rng().gen_range(self.perturb_min..self.perturb_max);
        while i < 0.5 {
            let transition = neighborhood.next_random();
            neighborhood.apply_transition(transition, &mut solution);
            i += 1.0;
        }
        solution
    }
}

impl Solver<TSPInstance, TSPSolution> for IteratedSolver {
    fn solve(&self, start_vertex: usize, instance: &TSPInstance) -> TSPSolution {
        let start = Instant::now();
        let mut solution = self.initial_solver.solve(start_vertex, instance);
        let mut solution = self.sub_solver.solve_s(start_vertex, instance, solution);
        let mut best_solution: (f32, TSPSolution) = (instance.eval(&solution), solution);
        let mut neighborhood = Neighborhood::new((self.transition)(), &best_solution.1, true);
        while start.elapsed().as_secs_f32() * 1000.0 < self.time {
            let mut perturb_solution = self.perturb(&instance, best_solution.1.deep_clone(), &mut neighborhood);
            perturb_solution.reorder();
            let mut it_solution = self.sub_solver.solve_s(start_vertex, instance, perturb_solution);
            let mut it_score = instance.eval(&it_solution);
            if it_score < best_solution.0 {
                best_solution = (it_score, it_solution);
            }
        }
        best_solution.1
    }

    fn solve_s(&self, start_vertex: usize, instance: &TSPInstance, solution: TSPSolution) -> TSPSolution {
        unimplemented!()
    }
}

pub struct IteratedConstructionSolver {
    initial_solver: Box<dyn Solver<TSPInstance, TSPSolution>>,
    sub_solver: Box<dyn Solver<TSPInstance, TSPSolution>>,
    time: f32,
    perturb_min: f32,
    perturb_max: f32,
    transition: fn() -> Vec<Box<dyn Transition>>,
}


impl IteratedConstructionSolver {
    pub fn new(initial_solver: Box<dyn Solver<TSPInstance, TSPSolution>>,
               sub_solver: Box<dyn Solver<TSPInstance, TSPSolution>>,
               time: f32,
               perturb_min: f32,
               perturb_max: f32,
               transition: fn() -> Vec<Box<dyn Transition>>) -> IteratedConstructionSolver {
        IteratedConstructionSolver {
            initial_solver,
            sub_solver,
            time,
            perturb_min,
            perturb_max,
            transition,
        }
    }

    fn perturb(&self, instance: &TSPInstance, mut solution: TSPSolution, neighborhood: &mut Neighborhood) -> TSPSolution {
        let mut i: f32 = 0.0;
        let perturb_size = ((instance.dimension as f32) * thread_rng().gen_range(self.perturb_min..self.perturb_max)) as usize / 2;
        solution.perm_a = solution.perm_a
            .choose_multiple(&mut rand::thread_rng(), solution.perm_a.len() - perturb_size).cloned().collect();
        solution.perm_b = solution.perm_b
            .choose_multiple(&mut rand::thread_rng(), solution.perm_b.len() - perturb_size).cloned().collect();
        solution
    }
}

impl Solver<TSPInstance, TSPSolution> for IteratedConstructionSolver {
    fn solve(&self, start_vertex: usize, instance: &TSPInstance) -> TSPSolution {
        let start = Instant::now();
        let mut solution = self.initial_solver.solve(start_vertex, instance);
        let mut best_solution: (f32, TSPSolution) = (instance.eval(&solution), solution);
        let mut neighborhood = Neighborhood::new((self.transition)(), &best_solution.1, true);
        while start.elapsed().as_secs_f32() * 1000.0 < self.time {
            let mut perturb_solution = self.perturb(&instance, best_solution.1.deep_clone(), &mut neighborhood);
            let mut it_solution = self.sub_solver.solve_s(start_vertex, instance, perturb_solution);
            let mut it_score = instance.eval(&it_solution);
            if it_score < best_solution.0 {
                best_solution = (it_score, it_solution);
                println!("x")
            }
        }
        best_solution.1
    }

    fn solve_s(&self, start_vertex: usize, instance: &TSPInstance, solution: TSPSolution) -> TSPSolution {
        unimplemented!()
    }
}