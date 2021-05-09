use crate::tsp::neighborhoods::neighborhood::Neighborhood;
use std::time::Instant;
use std::cmp;
use std::cmp::Ordering;
use std::collections::BinaryHeap;

use crate::tsp::neighborhoods::transition::Transition;
use crate::tsp::neighborhoods::inter_cycle_transition::InterCycleTransition;
use crate::tsp::neighborhoods::edges_transition::EdgesTransition;
use crate::tsp::def::TSPSolution;
use crate::tsp::def::TSPInstance;
use crate::traits::{Solver, Instance};

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    a_prev: usize,
    a: usize,
    a_next: usize,
    b_prev: usize,
    b: usize,
    b_next: usize,
    inter_cycle: bool,
    score: isize,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.score.cmp(&self.score)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub struct MemorySolver {
    initial_solver: Box<dyn Solver<TSPInstance, TSPSolution>>,
}

impl MemorySolver {
    pub fn new(initial_solver: Box<dyn Solver<TSPInstance, TSPSolution>>) -> MemorySolver {
        MemorySolver {
            initial_solver
        }
    }

    fn get_new_inter_cycle_state(&self, a_prev: usize, a: usize, a_next: usize, b_prev: usize, b: usize, b_next: usize, instance: &TSPInstance) -> State {
        let mut s = State{a_prev, a, a_next, b_prev, b, b_next, inter_cycle: true, score: 0};
        s.score = self.get_score(&s, instance);
        s
    }

    fn get_new_edges_state(&self, a: usize, a_next: usize, b: usize, b_next: usize, instance: &TSPInstance) -> State {
        let mut s = State{a_prev: 0, a, a_next, b_prev: 0, b, b_next, inter_cycle: false, score: 0};
        s.score = self.get_score(&s, instance);
        s
    }

    fn get_neighbors_in_cycle(&self, id: usize, perm:  &Vec<usize>) -> (usize, usize, usize) {
        let n = perm.len();
        (perm[(id+n-1)%n], perm[id], perm[(id+1)%n])
    }

    fn get_score(&self, s: &State, instance: &TSPInstance) -> isize {
        if s.inter_cycle {
            (instance.dist_k(s.b_prev, s.a) + instance.dist_k(s.a, s.b_next)
            + instance.dist_k(s.a_prev, s.b) + instance.dist_k(s.b, s.a_next)
            - instance.dist_k(s.a_prev, s.a) - instance.dist_k(s.a, s.a_next)
            - instance.dist_k(s.b_prev, s.b) - instance.dist_k(s.b, s.b_next)) as isize
        } else {
            (instance.dist_k(s.a, s.b) + instance.dist_k(s.a_next, s.b_next)
            - instance.dist_k(s.a, s.a_next) - instance.dist_k(s.b, s.b_next)) as isize
        }
    }

    fn is_valid(&self, s: &State, instance: &TSPInstance, solution: &TSPSolution) -> (Option<State>, usize) {
        let (a_prev, a, a_next) = self.get_neighbors_in_cycle(solution.order[s.a], if solution.cycle[s.a] == 0 { &solution.perm_a } else { &solution.perm_b });
        let (b_prev, b, b_next) = self.get_neighbors_in_cycle(solution.order[s.b], if solution.cycle[s.b] == 0 { &solution.perm_a } else { &solution.perm_b });
        if s.inter_cycle {
            let ns = self.get_new_inter_cycle_state(a_prev, a, a_next, b_prev, b, b_next, instance);
            if solution.cycle[s.a] == solution.cycle[s.b] {
                (None, 0)
            } else if  ns.score == s.score {
                (Some(ns), 2)
            } else {
                (None, 0)
            }
        } else {
            if solution.cycle[s.a] != solution.cycle[s.b] {
                (None, 0)
            } else if a_next == s.a_next && b_next == s.b_next {
                (Some(*s), 2)
            } else if a_prev == s.a_next && b_prev == s.b_next {
                (Some(self.get_new_edges_state(a_prev, a, b_prev, b, instance)), 2)
            } else if (a_prev == s.a_next && b_next == s.b_next) || (a_next == s.a_next && b_prev == s.b_next) {
                (Some(*s), 1)
            } else {
                (None, 0)
            }
        }
    }

    fn apply(&self, s: &State, instance: &TSPInstance, solution: &mut TSPSolution, q: &mut BinaryHeap<State>) {
        let (a_prev, a, a_next) = self.get_neighbors_in_cycle(solution.order[s.a], if solution.cycle[s.a] == 0 { &solution.perm_a } else { &solution.perm_b });
        let (b_prev, b, b_next) = self.get_neighbors_in_cycle(solution.order[s.b], if solution.cycle[s.b] == 0 { &solution.perm_a } else { &solution.perm_b });    
        let vertices_to_update;
        let edges_to_update;
        if s.inter_cycle {
            assert_ne!(solution.cycle[s.a], solution.cycle[s.b]);
            let inter_cycle_transition = InterCycleTransition::new();
            if solution.cycle[s.a] == 0 {
                inter_cycle_transition.apply_explicit(solution.order[s.a], solution.order[s.b], solution);
            } else {
                inter_cycle_transition.apply_explicit(solution.order[s.b], solution.order[s.a], solution);
            }
            vertices_to_update = vec![a_prev, a, a_next, b_prev, b, b_next];
            edges_to_update = vec![(a_prev, b), (b, a_next), (b_prev, a), (a, b_next)];
        } else  {
            assert_eq!(solution.cycle[s.a], solution.cycle[s.b]);
            let edges_transition = EdgesTransition::new();
            edges_transition.apply_explicit(solution.cycle[s.a], solution.order[s.a], solution.order[s.b], solution);
            vertices_to_update = vec![a, a_next, b, b_next];
            edges_to_update = vec![(a, b), (a_next, b_next)];
        }
        for edge in edges_to_update.iter() {
            self.update_edge(*edge, instance, solution, q);
        }
        for vertex in vertices_to_update.iter() {
            self.update_vertex(*vertex, instance, solution, q);
        }
    }

    fn update_edge(&self, edge: (usize, usize), instance: &TSPInstance, solution: &TSPSolution, q: &mut BinaryHeap<State>) {
        for b in 0..instance.dimension {
            if solution.cycle[edge.0] == solution.cycle[b] {
                let (_, b, b_next) = self.get_neighbors_in_cycle(solution.order[b], if solution.cycle[b] == 0 { &solution.perm_a } else { &solution.perm_b });
                if b != edge.0 && b != edge.1 && b_next != edge.0 && b_next != edge.1 {
                    let s = self.get_new_edges_state(edge.0, edge.1, b, b_next, instance);
                    if s.score < 0 {
                        q.push(s);
                    }
                    let ns = self.get_new_edges_state(edge.0, edge.1, b_next, b, instance);
                    if ns.score < 0 {
                        q.push(ns);
                    }
                } 
            }
        }
    }

    fn update_vertex(&self, a: usize, instance: &TSPInstance, solution: &TSPSolution, q: &mut BinaryHeap<State>) {
        let (a_prev, a, a_next) = self.get_neighbors_in_cycle(solution.order[a], if solution.cycle[a] == 0 { &solution.perm_a } else { &solution.perm_b });
        for b in 0..instance.dimension {
            if solution.cycle[a] != solution.cycle[b] {
                let (b_prev, b, b_next) = self.get_neighbors_in_cycle(solution.order[b], if solution.cycle[b] == 0 { &solution.perm_a } else { &solution.perm_b });
                let s = self.get_new_inter_cycle_state(a_prev, a, a_next, b_prev, b, b_next, instance);
                if s.score < 0 {
                    q.push(s);
                }
            }
        }
    }

}

impl Solver<TSPInstance, TSPSolution> for MemorySolver {
    fn solve(&self, start_vertex: usize, instance: &TSPInstance) -> TSPSolution {
        let mut solution = self.initial_solver.solve(start_vertex, instance);
        self.solve_s(start_vertex, instance, solution)
    }

    fn solve_s(&self, start_vertex: usize, instance: &TSPInstance, mut solution: TSPSolution) -> TSPSolution {
        let mut improvement_flag = true;
        let mut q: BinaryHeap<State> = BinaryHeap::new();
        for i in 0..instance.dimension {
            self.update_vertex(i, instance, &solution, &mut q);
            let (_, _, i_next) = self.get_neighbors_in_cycle(solution.order[i], if solution.cycle[i] == 0 { &solution.perm_a } else { &solution.perm_b });
            self.update_edge((i, i_next), instance, &solution, &mut q);
        }
        while improvement_flag {
            //assert begin
            // let mut neighborhood = Neighborhood::new(vec![Box::new(EdgesTransition{})], &solution, false);
            // let mut neighborhood = Neighborhood::new(vec![Box::new(EdgesTransition{}), Box::new(InterCycleTransition{})], &solution, false);
            // neighborhood.reset();
            // let mut best_move: (f32, usize) = (1.0, 0);
            // loop {
            //     let score = neighborhood.next(instance, &solution);
            //     match score {
            //         Some(x) => {
            //             if x.0 < best_move.0 {
            //                 best_move = x
            //             }
            //         },
            //         None => break
            //     }
            // }
            // if best_move.0 < 0.0 {
            //     neighborhood.show_transition(best_move.1, &solution);
            // }
            //assert end

            improvement_flag = false;
            let mut to_add: Vec<State> = Vec::new();
            while let Some(s) = q.pop() {
                let (s, c) = self.is_valid(&s, &instance, &solution);
                if c == 0 {
                    continue;
                }
                let s = s.unwrap();
                if c == 2 {
                    improvement_flag = true;
                    for ss in to_add.iter() {
                        q.push(*ss);
                    }
                    // let s1 = instance.eval(&solution);
                    // assert_eq!(s.score, best_move.0 as isize);
                    self.apply(&s, instance, &mut solution, &mut q);
                    // let s2 = instance.eval(&solution);
                    // assert_eq!((s2-s1).round() as isize, s.score);
                    break;
                } else if c == 1 {
                    to_add.push(s);
                }
            }
        }
        solution
    }
}

