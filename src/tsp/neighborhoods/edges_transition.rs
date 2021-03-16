use crate::tsp::def::TSPSolution;
use crate::tsp::def::TSPInstance;
use crate::tsp::neighborhoods::transition::Transition;

pub struct EdgesTransition {
    // cycle: usize,
    // vertex: usize,
    // shift: usize,
    // prev_cycle: usize,
    // prev_vertex: usize,
    // prev_shift: usize
}

impl EdgesTransition {
    pub fn new() -> EdgesTransition {
        EdgesTransition {
            // cycle: 0,
            // vertex: 0,
            // shift: 1,
            // prev_cycle: 0,
            // prev_vertex: 0,
            // prev_shift: 0
        }
    }
    fn unpack_state(&self, state: usize, solution: &TSPSolution) -> Option<(usize, usize, usize)> {
        let mut state = state;
        state-=1;
        let (n_a, n_b) = (solution.perm_a.len(), solution.perm_b.len());
        if n_a*(n_a-3) > state { //first cycle
            Some((0, state/(n_a-3), state%(n_a-3)))
        } else if n_a*(n_a-3) + n_b*(n_b-3) > state {
            state -= n_a*(n_a-3);
            Some((1, state/(n_b-3), state%(n_b-3)))
        } else {
            return None
        }
    }
}

impl Transition for EdgesTransition {

    fn size(&self, solution: &TSPSolution) -> usize {
        let (n_a, n_b) = (solution.perm_a.len(), solution.perm_b.len());
        n_a*(n_a-3) + n_b*(n_b-3)
    }

    fn score(&self, state: usize, instance: &TSPInstance, solution: &TSPSolution) -> Option<f32> {
        let st = self.unpack_state(state, solution);
        if st.is_none() {
            return None
        }
        let (cycle, vertex, shift) = st.unwrap();
        let perm = vec![&solution.perm_a, &solution.perm_b];
        let vertex_b = (vertex+shift)%perm[cycle].len();
        let (a_prev, a, _) = self.get_neighbors_in_cycle(vertex, perm[cycle]);
        let (_, b, b_next) = self.get_neighbors_in_cycle(vertex_b, perm[cycle]);
        let delta = instance.dist_k(a, b_next) + instance.dist_k(a_prev, b)
            - instance.dist_k(a_prev, a) - instance.dist_k(b, b_next);
        Some(delta)
    }

    fn apply(&self, state: usize, solution: &mut TSPSolution) {
        let (cycle, vertex, shift) = self.unpack_state(state, solution).unwrap();
        let perm = if cycle == 0 { &mut solution.perm_a } else { &mut solution.perm_b };
        let mut a = vertex;
        let mut b = (a+shift)%perm.len();
        if b < a {
            b+=1;
            a-=1;
            let t = b; b = a; a = t;
        }
        while a < b {
            let t = perm[a]; perm[a] = perm[b]; perm[b] = t;
            a+=1;
            b-=1;
        }
    }

    // fn next(&mut self, instance: &TSPInstance, solution: &TSPSolution) -> Option<f32> {
    //     if self.cycle == 2 {
    //         return None;
    //     }
    //     let perm = vec![&solution.perm_a, &solution.perm_b];
    //     let a = perm[self.cycle][self.vertex];
    //     let vertex_b = (self.vertex+self.shift)%perm[self.cycle].len();
    //     let b = perm[self.cycle][vertex_b];
    //     let (a_prev, _) = self.get_neighbors_in_cycle(self.vertex, perm[self.cycle]);
    //     let (_, b_next) = self.get_neighbors_in_cycle(vertex_b, perm[self.cycle]);
    //     let delta = instance.dist_k(a, b_next) + instance.dist_k(a_prev, b)
    //         - instance.dist_k(a_prev, a) - instance.dist_k(b, b_next);
    //     self.prev_cycle = self.cycle;
    //     self.prev_vertex = self.vertex;
    //     self.prev_shift = self.shift;
    //     if self.shift+3 == perm[self.cycle].len() {
    //         self.vertex+=1;
    //         self.shift = 1;
    //         if self.vertex == perm[self.cycle].len() {
    //             self.cycle+=1;
    //             self.vertex = 0;
    //             self.shift = 1;
    //         }
    //     }
    //     Some(delta)
    // }
    // fn random(&mut self, instance: &TSPInstance, solution: &TSPSolution) -> Option<f32> {
    //     let mut rng = rand::thread_rng();
    //     self.cycle = rng.gen_range(0..2);
    //     let n = if self.cycle == 0 {solution.perm_a.len()} else {solution.perm_b.len()};
    //     self.vertex = rng.gen_range(0..n);
    //     self.shift = rng.gen_range(1..n-2);
    //     self.next(instance, solution)
    // }
    // fn apply_last(&self, instance: &TSPInstance, solution: &mut TSPSolution) {
        
    // }
    // fn reset(&mut self) {
    //     self.cycle = 0;
    //     self.vertex = 0;
    //     self.shift = 1;
    //     self.prev_cycle = 0;
    //     self.prev_vertex = 0;
    //     self.prev_shift = 0;
    // }
}