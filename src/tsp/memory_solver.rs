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

    fn get_neighbors_in_cycle(&self, id: usize, perm:  &Vec<usize>) -> (usize, usize, usize) {
        let n = perm.len();
        (perm[(id+n-1)%n], perm[id], perm[(id+1)%n])
    }

    fn get_score(&self, s: &State, instance: &TSPInstance, solution: &TSPSolution) -> isize {
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

    fn is_valid(&self, s: &State, instance: &TSPInstance, solution: &TSPSolution) -> usize {
        let (a_prev, a, a_next) = self.get_neighbors_in_cycle(solution.order[s.a], if solution.cycle[s.a] == 0 { &solution.perm_a } else { &solution.perm_b });
        let (b_prev, b, b_next) = self.get_neighbors_in_cycle(solution.order[s.b], if solution.cycle[s.b] == 0 { &solution.perm_a } else { &solution.perm_b });
        if s.inter_cycle {
            if solution.cycle[s.a] == solution.cycle[s.b] {
                0
            } else if a_prev == s.a_prev && a == s.a && a_next == s.a_next && b_prev == s.b_prev && b == s.b && b_next == s.b_next {
                2
            } else {
                0
            }
        } else {
            if solution.cycle[s.a] != solution.cycle[s.b] {
                0
            } else if a == s.a && a_next == s.a_next && b == s.b && b_next == s.b_next {
                2
            } else if (a_next == s.a_next && b_prev == s.b_next) || (a_prev == s.a_next && b_next == s.b_next) {
                1
            } else {
                0
            }
        }
    }

    fn apply(&self, s: &State, instance: &TSPInstance, solution: &mut TSPSolution, q: &mut BinaryHeap<State>) {
        if s.inter_cycle {
            let (a_prev, a, a_next) = self.get_neighbors_in_cycle(solution.order[s.a], if solution.cycle[s.a] == 0 { &solution.perm_a } else { &solution.perm_b });
            let (b_prev, b, b_next) = self.get_neighbors_in_cycle(solution.order[s.b], if solution.cycle[s.b] == 0 { &solution.perm_a } else { &solution.perm_b });
            let inter_cycle_transition = InterCycleTransition::new();
            inter_cycle_transition.apply_explicit(solution.order[s.a], solution.order[s.b], solution);
            self.update_vertex(a_prev, instance, solution, q);
            self.update_vertex(a, instance, solution, q);
            self.update_vertex(a_next, instance, solution, q);
            self.update_vertex(b_prev, instance, solution, q);
            self.update_vertex(b, instance, solution, q);
            self.update_vertex(b_next, instance, solution, q);
        } else  {
            let edges_transition = EdgesTransition::new();
            edges_transition.apply_explicit(solution.cycle[s.a], solution.order[s.a], solution.order[s.b], solution);
        }
        
    }

    fn update_vertex(&self, a: usize, instance: &TSPInstance, solution: &TSPSolution, q: &mut BinaryHeap<State>) {
        let (a_prev, a, a_next) = self.get_neighbors_in_cycle(solution.order[a], if solution.cycle[a] == 0 { &solution.perm_a } else { &solution.perm_b });
        for b in 0..instance.dimension {
            if b == a {
                continue;
            }
            if solution.cycle[a] == solution.cycle[b] {
                let (b_prev, b, b_next) = self.get_neighbors_in_cycle(solution.order[b], if solution.cycle[b] == 0 { &solution.perm_a } else { &solution.perm_b });
                if b == a_next || b_next == a {
                    continue
                }
                let mut s = State {
                    score: 0, a_prev, a, a_next, b_prev, b, b_next, inter_cycle: false
                };
                s.score = self.get_score(&s, instance, solution);
                if s.score <  0 {
                    q.push(s);
                }
                let mut s = State {
                    score: 0, a_prev, a, a_next, b_prev, b_next, b, inter_cycle: false
                };
                s.score = self.get_score(&s, instance, solution);
                if s.score <  0 {
                    q.push(s);
                }

            } else {
                let (b_prev, b, b_next) = self.get_neighbors_in_cycle(solution.order[b], if solution.cycle[b] == 0 { &solution.perm_a } else { &solution.perm_b });
                let mut s = State {
                    score: 0, a_prev, a, a_next, b_prev, b, b_next, inter_cycle: true
                };
                s.score = self.get_score(&s, instance, solution);
                if s.score <  0 {
                    q.push(s);
                }
            }
        }
    }

}

impl Solver<TSPInstance, TSPSolution> for MemorySolver {
    fn solve(&self, start_vertex: usize, instance: &TSPInstance) -> TSPSolution {
        let mut solution = self.initial_solver.solve(start_vertex, instance);
        let mut improvement_flag = true;
        let mut q: BinaryHeap<State> = BinaryHeap::new();
        for i in 0..instance.dimension {
            self.update_vertex(i, instance, &solution, &mut q);
        }
        while improvement_flag {
            improvement_flag = false;
            let mut to_add: Vec<State> = Vec::new();
            while let Some(s) = q.pop() {
                let c = self.is_valid(&s, &instance, &solution);
                if c == 2 {
                    improvement_flag = true;
                    for ss in to_add.iter() {
                        q.push(*ss);
                    }
                    self.apply(&s, instance, &mut solution, &mut q);
                    break;
                } else if c == 1 {
                    to_add.push(s);
                }
            }
        }
        solution
    }
}

