use crate::tsp::neighborhoods::transition::Transition;
use crate::tsp::def::{TSPInstance, TSPSolution};

pub struct InterCycleTransition {}

impl InterCycleTransition {
    pub fn new() -> InterCycleTransition {
        InterCycleTransition {}
    }
    fn unpack_state(&self, state: usize, solution: &TSPSolution) -> Option<(usize, usize)> {
        let mut state = state;
        state-=1;
        let (n_a, n_b) = (solution.perm_a.len(), solution.perm_b.len());
        let cycles = (state/n_b, state%n_b);
        if cycles.0 >= n_a {
            return None;
        }
        Some(cycles)
    }
}

impl Transition for InterCycleTransition {

    fn size(&self, solution: &TSPSolution) -> usize {
        let (n_a, n_b) = (solution.perm_a.len(), solution.perm_b.len());
        n_a*n_b
    }

    fn score(&self, state: usize, instance: &TSPInstance, solution: &TSPSolution) -> Option<f32> {
        let ids = self.unpack_state(state, &solution);
        if ids.is_none() {
            return None
        }
        let (cycle_a, cycle_b) = ids.unwrap();
        let (a_prev, a, a_next) = self.get_neighbors_in_cycle(cycle_a, &solution.perm_a);
        let (b_prev, b, b_next) = self.get_neighbors_in_cycle(cycle_b, &solution.perm_b);
        let delta = instance.dist_k(b_prev, a) + instance.dist_k(a, b_next)
            + instance.dist_k(a_prev, b) + instance.dist_k(b, a_next)
            - instance.dist_k(a_prev, a) - instance.dist_k(a, a_next)
            - instance.dist_k(b_prev, b) - instance.dist_k(b, b_next);
        Some(delta)
    }

    fn apply(&self, state: usize, solution: &mut TSPSolution) {
        let (cycle_a, cycle_b) = self.unpack_state(state, solution).unwrap();
        let t = solution.perm_a[cycle_a];
        solution.perm_a[cycle_a] = solution.perm_b[cycle_b];
        solution.perm_b[cycle_b] = t;
    }
}