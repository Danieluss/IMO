use crate::tsp::neighborhoods::transition::Transition;
use crate::tsp::def::{TSPInstance, TSPSolution};

pub struct InterCycleTransition {}
//     cycle_a: usize,
//     cycle_b: usize,
//     prev_cycle_a: usize,
//     prev_cycle_b: usize
// }

impl InterCycleTransition {
    pub fn new() -> InterCycleTransition {
        InterCycleTransition {
            // cycle_a: 0,
            // cycle_b: 0,
            // prev_cycle_a: 0,
            // prev_cycle_b: 0
        }
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


    // fn next(&mut self, instance: &TSPInstance, solution: &TSPSolution) -> Option<f32> {
    //     if self.cycle_a == solution.perm_a.len() {
    //         return None
    //     }
    //     let (a_prev, a, a_next) = self.get_neighbors_in_cycle(self.cycle_a, &solution.perm_a);
    //     let (b_prev, b, b_next) = self.get_neighbors_in_cycle(self.cycle_b, &solution.perm_b);
    //     let delta = instance.dist_k(b_prev, a) + instance.dist_k(a, b_next)
    //         + instance.dist_k(a_prev, b) + instance.dist_k(b, a_next)
    //         - instance.dist_k(a_prev, a) - instance.dist_k(a, a_next)
    //         - instance.dist_k(b_prev, b) - instance.dist_k(b, b_next);
    //     self.prev_cycle_a = self.cycle_a;
    //     self.prev_cycle_b = self.cycle_b;
    //     self.cycle_b+=1;
    //     if self.cycle_b == solution.perm_b.len() {
    //         self.cycle_a += 1;
    //         self.cycle_b = 0;
    //     }
    //     Some(delta)
    // }
    // fn random(&mut self, instance: &TSPInstance, solution: &TSPSolution) -> Option<f32> {
    //     let mut rng = rand::thread_rng();
    //     self.cycle_a = rng.gen_range(0..solution.perm_a.len());
    //     self.cycle_b = rng.gen_range(0..solution.perm_b.len());
    //     self.next(instance, solution)
    // }
    // fn apply_last(&self, instance: &TSPInstance, solution: &mut TSPSolution) {

    // }
    // fn reset(&mut self) {
    //     self.cycle_a = 0;
    //     self.cycle_b = 0;
    //     self.prev_cycle_a = 0;
    //     self.prev_cycle_b = 0;
    // }
}