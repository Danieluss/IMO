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


pub struct Population {
    pop_registry: Vec<Candidate>,
    pop_queue: BinaryHeap<(MinFloat, usize)>,
    score_map: HashMap<usize, usize>,
}

impl Population {
    pub fn new() -> Population {
        let mut pop_registry = vec![];
        let mut pop_queue = BinaryHeap::new();
        let mut score_map = HashMap::new();

        Population {
            pop_registry,
            pop_queue,
            score_map,
        }
    }

    pub fn register(&mut self, candidate: Candidate) {
        let position = self.pop_registry.len();
        self.score_map.insert(candidate.distance.clone() as usize, position);
        self.pop_queue.push((MinFloat(candidate.distance.clone()), position));
        self.pop_registry.push(candidate);
    }

    pub fn inbred(&mut self, candidate: &Candidate) {
        let index = self.score_map.get(&(candidate.distance as usize)).unwrap();
        self.pop_registry[*index].inbred_count += 1;
        self.pop_registry[*index].cross_count += 1;
    }

    pub fn should_include(&self, candidate: &Candidate) -> bool {
        self.pop_queue.peek().unwrap().0.0 > candidate.distance
    }

    pub fn replace(&mut self, candidate: Candidate) {
        let popped_candidate = self.pop_queue.pop().unwrap();
        self.score_map.remove(&(self.pop_registry[popped_candidate.1].distance as usize));
        self.pop_registry[popped_candidate.1] = candidate;
        self.pop_queue.push((MinFloat(self.pop_registry[popped_candidate.1].distance.clone()), popped_candidate.1));
        self.score_map.insert(self.pop_registry[popped_candidate.1].distance.clone() as usize, popped_candidate.1);
    }

    pub fn threshold(&self) -> f32 {
        self.pop_queue.peek().unwrap().0.0
    }

    pub fn has(&self, candidate: &Candidate) -> bool {
        self.score_map.contains_key(&(candidate.distance as usize))
    }

    pub fn get(&self, index: usize) -> &Candidate {
        &self.pop_registry[index]
    }

    pub fn get_mut(&mut self, index: usize) -> &mut Candidate {
        &mut self.pop_registry[index]
    }

    pub fn size(&self) -> usize {
        self.pop_registry.len()
    }

    pub fn rehash(&mut self) {
        self.pop_queue.clear();
        self.score_map.clear();
        for i in 0..self.size() {
            self.pop_queue.push((MinFloat(self.pop_registry[i].distance), i));
            self.score_map.insert(self.pop_registry[i].distance as usize, i);
        }
    }

    pub fn join(&mut self, population: &mut Population) {
        for i in 0..population.pop_registry.len() {
            if !self.has(&population.pop_registry[0]) {
                self.pop_registry.push(population.pop_registry.remove(0));
            }
        }
    }
}

pub struct CustomSolver {
    local_solver: Box<dyn Solver<TSPInstance, TSPSolution>>,
    construction_solver: Box<dyn Solver<TSPInstance, TSPSolution>>,
    time: f32,
    no_populations: usize,
    population_size: usize,
    steps_to_mutation: usize,
    transition: fn() -> Vec<Box<dyn Transition>>,
}

impl CustomSolver {
    pub fn new(local_solver: Box<dyn Solver<TSPInstance, TSPSolution>>,
               construction_solver: Box<dyn Solver<TSPInstance, TSPSolution>>,
               time: f32,
               no_populations: usize,
               population_size: usize,
               steps_to_mutation: usize,
               transition: fn() -> Vec<Box<dyn Transition>>) -> CustomSolver {
        CustomSolver {
            local_solver,
            construction_solver,
            time,
            no_populations,
            population_size,
            steps_to_mutation,
            transition,
        }
    }

    fn cross_over(&self, parents: (usize, usize), instance: &TSPInstance, population: &mut Population) -> Option<Candidate> {
        let candidate_a = population.get(parents.0);
        let candidate_b = population.get(parents.1);
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

        let mut solution = TSPSolution { perm_a: new_perm_a, perm_b: new_perm_b, cycle: vec![], order: vec![] };
        let mut solution = self.construction_solver.solve_s(0, instance, solution);
        solution.reorder();
        let mut solution = self.local_solver.solve_s(0, instance, solution);
        let distance = instance.eval(&solution);

        let f = distance == candidate_a.distance || distance == candidate_b.distance;

        if distance == population.get(parents.0).distance {
            population.get_mut(parents.0).inbred_count += 1;
        }
        if distance == population.get(parents.1).distance {
            population.get_mut(parents.1).inbred_count += 1;
        }
        population.get_mut(parents.0).cross_count += 1;
        population.get_mut(parents.1).cross_count += 1;

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

    fn mutate(&self, instance: &TSPInstance, population: &mut Population, neighborhood: &mut Neighborhood) {
        for candidate in population.pop_registry.iter_mut() {
            let inbreds = candidate.inbred_count as f32;
            let total = candidate.cross_count as f32;
            if candidate.cross_count != 0 && candidate.inbred_count != 0 {
                let mx = if inbreds / total + 0.1 < 0.8 { inbreds / total + 0.1 } else { 0.8 };
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

        population.rehash();
    }
}


impl Solver<TSPInstance, TSPSolution> for CustomSolver {
    fn solve(&self, _: usize, instance: &TSPInstance) -> TSPSolution {
        println!("=====");
        let EMPTY = TSPSolution {
            perm_a: vec![],
            perm_b: vec![],
            cycle: vec![],
            order: vec![],
        };
        let start = Instant::now();

        let mut populations = vec![];
        let mut best_solution: (f32, (usize, usize)) = (f32::INFINITY, (0, 0));
        for i in 0..self.no_populations {
            println!("New population");
            let mut population = Population::new();

            let start_each = instance.dimension / self.no_populations;
            let step_each = instance.dimension / self.no_populations / self.population_size;
            for start_i in 0..self.population_size / self.no_populations {
                let candidate = self.new_candidate(i * start_each + start_i * step_each, instance);
                if candidate.distance < best_solution.0 {
                    best_solution = (instance.eval(&candidate.solution), (i, start_i));
                    println!(">> {}", best_solution.0);
                }
                population.register(candidate);
            }

            populations.push(population);
        }

        let mut best_solution = (best_solution.0, populations[best_solution.1.0]
            .get(best_solution.1.0).solution.clone());
        let mut neighborhood = Neighborhood::new((self.transition)(), &best_solution.1, true);
        let island_steps = 4;
        while start.elapsed().as_secs_f32() * 1000.0 < self.time {
            for population_i in 0..populations.len() {
                let population = &mut populations[population_i];
                for island_step in 0..island_steps {
                    if population.size() * (population.size() - 1) / 2 < self.steps_to_mutation {
                        for parent_i in 0..population.size() - 1 {
                            for parent_j in parent_i + 1..population.size() {
                                match self.cross_over((parent_i, parent_j), instance, population) {
                                    Some(candidate) => {
                                        if population.has(&candidate) {
                                            population.inbred(&candidate);
                                        } else if population.should_include(&candidate) {
                                            println!("{} {}", candidate.distance, population.threshold());
                                            if candidate.distance < best_solution.0 {
                                                best_solution = (instance.eval(&candidate.solution), candidate.solution.clone());
                                                println!(">> {}", best_solution.0);
                                            }
                                            population.replace(candidate);
                                        }
                                    }
                                    None => {}
                                }
                                if start.elapsed().as_secs_f32() >= self.time {
                                    return best_solution.1;
                                }
                            }
                        }
                    } else {
                        for step_i in 0..self.steps_to_mutation {
                            match self.cross_over(random_combination(population.size()), instance, population) {
                                Some(candidate) => {
                                    if population.has(&candidate) {
                                        population.inbred(&candidate);
                                    } else if population.should_include(&candidate) {
                                        println!("{} {}", candidate.distance, population.threshold());
                                        if candidate.distance < best_solution.0 {
                                            best_solution = (instance.eval(&candidate.solution), candidate.solution.clone());
                                            println!(">> {}", best_solution.0);
                                        }
                                        population.replace(candidate);
                                    }
                                }
                                None => {}
                            }
                            if start.elapsed().as_secs_f32() >= self.time {
                                return best_solution.1;
                            }
                        }
                    }
                    if island_step != island_steps - 1 {
                        println!("Mutating");
                        self.mutate(instance, population, &mut neighborhood);
                    }
                }
            }
            println!("Joining");
            if populations.len() / 2 > 0 {
                let half_index = populations.len() / 2;
                for population_i in 0..half_index {
                    let mut population = populations.remove(half_index);
                    populations[population_i].join(&mut population);
                    while populations[population_i].size() < half_index * 2 {
                        populations[population_i].register(self.new_candidate(rand::thread_rng().gen_range(0..instance.dimension), instance))
                    }
                    populations[population_i].rehash();
                }
            }
        }

        best_solution.1
    }

    fn solve_s(&self, start_vertex: usize, instance: &TSPInstance, solution: TSPSolution) -> TSPSolution {
        unimplemented!()
    }
}
