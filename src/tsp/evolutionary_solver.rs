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

pub struct Candidate {
    solution: TSPSolution,
    distance: f32,
    inbred_count: usize,
    cross_count: usize,
}

impl Eq for Candidate {}

impl PartialEq for Candidate {
    fn eq(&self, other: &Self) -> bool {
        self.distance == other.distance
    }
}

impl Ord for Candidate {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .distance
            .partial_cmp(&self.distance)
            .unwrap()
    }
}

impl PartialOrd for Candidate {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}


pub struct EvolutionarySolver {
    local_solver: Box<dyn Solver<TSPInstance, TSPSolution>>,
    construction_solver: Box<dyn Solver<TSPInstance, TSPSolution>>,
    time: f32,
    population_size: usize,
    steps_to_mutation: usize,
    transition: fn() -> Vec<Box<dyn Transition>>,
}

impl EvolutionarySolver {
    pub fn new(local_solver: Box<dyn Solver<TSPInstance, TSPSolution>>,
               construction_solver: Box<dyn Solver<TSPInstance, TSPSolution>>,
               time: f32,
               population_size: usize,
               steps_to_mutation: usize,
               transition: fn() -> Vec<Box<dyn Transition>>) -> EvolutionarySolver {
        EvolutionarySolver {
            local_solver,
            construction_solver,
            time,
            population_size,
            steps_to_mutation,
            transition,
        }
    }

    fn cross_over(&self, parents: (usize, usize), instance: &TSPInstance, population: &mut Vec<Candidate>) -> Option<Candidate> {
        let candidate_a = population.get(parents.0).unwrap();
        let candidate_b = population.get(parents.1).unwrap();
        let mut new_perm_a = vec![];
        let mut new_perm_b = vec![];
        let mut noswap_count = 0;

        for vertex_i in 0..instance.dimension {
            if candidate_a.solution.cycle[vertex_i] == candidate_b.solution.cycle[vertex_i] {
                noswap_count += 1;
            }
        }
        let mut a_cycle_dest = 0;
        let mut a_perm_a = &candidate_a.solution.perm_a;
        let mut a_perm_b = &candidate_a.solution.perm_b;

        let b_perm_a = &candidate_b.solution.perm_a;
        let b_perm_b = &candidate_b.solution.perm_b;
        let b_order = &candidate_b.solution.order;
        let b_cycle = &candidate_b.solution.cycle;

        if noswap_count < instance.dimension / 2 {
            a_perm_a = &candidate_a.solution.perm_b;
            a_perm_b = &candidate_a.solution.perm_a;
            a_cycle_dest = 1;
        }
        for a_vert_i in 0..a_perm_a.len() {
            let g_vert = a_perm_a[a_vert_i];
            if b_cycle[g_vert] == a_cycle_dest
                && (b_perm_a[(b_order[g_vert] + 1) % a_perm_a.len()] == a_perm_a[(a_vert_i + 1) % a_perm_a.len()]
                || b_perm_a[(b_order[g_vert] + a_perm_a.len() - 1) % a_perm_a.len()] == a_perm_a[(a_vert_i + 1) % a_perm_a.len()]) {
                let a = g_vert;
                if new_perm_a.is_empty() || a != *new_perm_a.last().unwrap() {
                    new_perm_a.push(g_vert);
                }

                let a = a_perm_a[(a_vert_i + 1) % a_perm_a.len()];
                if new_perm_a.is_empty() || a != *new_perm_a.first().unwrap() {
                    new_perm_a.push(a);
                }
            }
        }

        for a_vert_i in 0..a_perm_b.len() {
            let g_vert = a_perm_b[a_vert_i];
            if b_cycle[g_vert] == 1 - a_cycle_dest
                && (b_perm_b[(b_order[g_vert] + 1) % b_perm_b.len()] == a_perm_b[(a_vert_i + 1) % a_perm_b.len()]
                || b_perm_b[(b_order[g_vert] + b_perm_b.len() - 1) % b_perm_b.len()] == a_perm_b[(a_vert_i + 1) % a_perm_b.len()]) {
                let a = g_vert;
                if new_perm_b.is_empty() || a != *new_perm_b.last().unwrap() {
                    new_perm_b.push(g_vert);
                }

                let a = a_perm_b[(a_vert_i + 1) % a_perm_b.len()];
                if new_perm_b.is_empty() || a != *new_perm_b.first().unwrap() {
                    new_perm_b.push(a);
                }
            }
        }

        let mut solution = TSPSolution{ perm_a: new_perm_a, perm_b: new_perm_b, cycle: vec![], order: vec![] };
        let mut solution = self.construction_solver.solve_s(0, instance, solution);
        solution.reorder();
        // let mut solution = self.local_solver.solve_s(0, instance, solution);
        let distance = instance.eval(&solution);

        let f = distance == candidate_a.distance || distance == candidate_b.distance;

        if distance == population[parents.0].distance {
            population[parents.0].inbred_count += 1;
        }
        if distance == population[parents.1].distance {
            population[parents.1].inbred_count += 1;
        }
        population[parents.0].cross_count += 1;
        population[parents.1].cross_count += 1;

        if f {
            None
        } else {
            Some(Candidate {
                solution,
                distance,
                inbred_count: 0,
                cross_count: 0,
            })
        }
    }

    fn new_candidate(&self, start_vertex: usize, instance: &TSPInstance) -> Candidate {
        let solution = self.construction_solver.solve(4 + start_vertex, instance);
        let solution = self.local_solver.solve_s(start_vertex, instance, solution);
        let distance = instance.eval(&solution);
        let inbred_count = 0;
        let cross_count = 0;

        Candidate {
            solution,
            distance,
            inbred_count,
            cross_count,
        }
    }

    fn perturb(&self, instance: &TSPInstance, mut solution: TSPSolution, neighborhood: &mut Neighborhood, perturb_size: usize) -> TSPSolution {
        solution.perm_a = solution.perm_a
            .choose_multiple(&mut rand::thread_rng(), solution.perm_a.len() - perturb_size).cloned().collect();
        solution.perm_b = solution.perm_b
            .choose_multiple(&mut rand::thread_rng(), solution.perm_b.len() - perturb_size).cloned().collect();
        solution
    }

    fn mutate(&self, instance: &TSPInstance, population: &mut Vec<Candidate>, neighborhood: &mut Neighborhood) {
        for candidate in population.iter_mut() {
            let inbreds = candidate.inbred_count as f32;
            let total = candidate.cross_count as f32;
            if candidate.cross_count != 0 && candidate.inbred_count != 0{
                let mx =  if inbreds/total + 0.1 < 1. {inbreds/total + 0.1} else {1.};
                let perturb_size = ((instance.dimension as f32) * thread_rng().gen_range(0.1..mx)) as usize / 2;
                let solution = self.perturb(instance, candidate.solution.clone(), neighborhood, perturb_size);
                let solution = self.construction_solver.solve_s(0, instance, solution);
                candidate.solution = solution;
                candidate.solution.reorder();
                candidate.distance = instance.eval(&candidate.solution);
                candidate.inbred_count = 0;
                candidate.cross_count = 0;
            }
        }
    }
}


impl Solver<TSPInstance, TSPSolution> for EvolutionarySolver {
    fn solve(&self, _: usize, instance: &TSPInstance) -> TSPSolution {
        println!("=====");
        let EMPTY = TSPSolution {
            perm_a: vec![],
            perm_b: vec![],
            cycle: vec![],
            order: vec![],
        };
        let start = Instant::now();
        let mut best_solution: (f32, TSPSolution) = (f32::INFINITY, EMPTY);

        let mut pop_registry = vec![];
        let mut pop_queue = BinaryHeap::new();
        let mut score_map = HashMap::new();

        let start_each = instance.dimension / self.population_size;
        for start_i in 0..self.population_size {
            let candidate = self.new_candidate(start_i * start_each, instance);
            if candidate.distance < best_solution.0 {
                best_solution = (instance.eval(&candidate.solution), candidate.solution.clone());
                println!(">> {}", best_solution.0);
            }
            score_map.insert(candidate.distance.clone() as usize, pop_registry.len());
            pop_registry.push(candidate);
            pop_queue.push((MinFloat(pop_registry.last().unwrap().distance.clone()), pop_registry.len() - 1));
        }
        let mut neighborhood = Neighborhood::new((self.transition)(), &best_solution.1, true);
        while start.elapsed().as_secs_f32() * 1000.0 < self.time {
            for i in 0..self.steps_to_mutation {
                match self.cross_over(random_combination(self.population_size), instance, &mut pop_registry) {
                    Some(candidate) => {
                        if score_map.contains_key(&(candidate.distance as usize)) {
                            let index = score_map.get(&(candidate.distance as usize)).unwrap();
                            pop_registry[*index].inbred_count += 1;
                            pop_registry[*index].cross_count += 1;
                        } else if pop_queue.peek().unwrap().0.0 > candidate.distance {
                            println!("{} {}", candidate.distance, pop_queue.peek().unwrap().0.0);
                            let popped_candidate = pop_queue.pop().unwrap();
                            if candidate.distance < best_solution.0 {
                                best_solution = (instance.eval(&candidate.solution), candidate.solution.clone());
                                println!(">> {}", best_solution.0);
                            }
                            pop_registry[popped_candidate.1] = candidate;
                            pop_queue.push((MinFloat(pop_registry[popped_candidate.1].distance.clone()), popped_candidate.1));
                            score_map.insert(pop_registry[popped_candidate.1].distance.clone() as usize, popped_candidate.1);
                        }
                    }
                    None => {}
                }
                if start.elapsed().as_secs_f32() >= self.time {
                    return best_solution.1;
                }

            }
            println!("Mutating");
            self.mutate(instance, &mut pop_registry, &mut neighborhood);
            pop_queue.clear();
            score_map.clear();
            for i in 0..self.population_size {
                pop_queue.push((MinFloat(pop_registry[i].distance), i));
                score_map.insert(pop_registry[i].distance as usize, i);
            }
        }

        best_solution.1
    }

    fn solve_s(&self, start_vertex: usize, instance: &TSPInstance, solution: TSPSolution) -> TSPSolution {
        unimplemented!()
    }
}
