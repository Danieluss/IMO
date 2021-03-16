use crate::tsp::neighborhoods::transition::Transition;
use crate::tsp::def::TSPSolution;
use crate::tsp::def::TSPInstance;

pub struct VertexTransition {
    // cycle: usize,
    // vertex_a: usize,
    // vertex_b: usize,
    // prev_cycle: usize,
    // prev_vertex_a: usize,
    // prev_vertex_b: usize
}

impl VertexTransition {
    pub fn new() -> VertexTransition {
        VertexTransition {
            // cycle: 0,
            // vertex_a: 0,
            // vertex_b: 1,
            // prev_cycle: 0,
            // prev_vertex_a: 0,
            // prev_vertex_b: 0
        }
    }
    fn unpack_state(&self, state: usize, solution: &TSPSolution) -> Option<(usize, usize, usize)> {
        let mut state = state;
        state-=1;
        let (n_a, n_b) = (solution.perm_a.len(), solution.perm_b.len());
        if n_a*(n_a-1) > state { //first cycle
            let a = state/(n_a-1);
            let mut b = state%(n_a-1);
            if b >= a {
                b+=1;
            } 
            Some((0, a, b))
        } else if n_a*(n_a-1) + n_b*(n_b-1) > state {
            state -= n_a*(n_a-1);
            let a = state/(n_b-1);
            let mut b = state%(n_b-1);
            if b >= a {
                b+=1;
            }
            Some((1, a, b))
        } else {
            return None
        }
    }
}

impl Transition for VertexTransition {

    fn size(&self, solution: &TSPSolution) -> usize {
        let (n_a, n_b) = (solution.perm_a.len(), solution.perm_b.len());
        n_a*(n_a-1) + (n_b)*(n_b-1)
    }
    
    fn score(&self, state: usize, instance: &TSPInstance, solution: &TSPSolution) -> Option<f32> {
        let st = self.unpack_state(state, solution);
        if st.is_none() {
            return None;
        }
        let (cycle, vertex_a, vertex_b) = st.unwrap();
        let perm = vec![&solution.perm_a, &solution.perm_b];
        let (a_prev, a, a_next) = self.get_neighbors_in_cycle(vertex_a, perm[cycle]);
        let (b_prev, b, b_next) = self.get_neighbors_in_cycle(vertex_b, perm[cycle]);
        let mut delta = instance.dist_k(b_prev, a) + instance.dist_k(a, b_next)
            + instance.dist_k(a_prev, b) + instance.dist_k(b, a_next)
            - instance.dist_k(a_prev, a) - instance.dist_k(a, a_next)
            - instance.dist_k(b_prev, b) - instance.dist_k(b, b_next);
        if (vertex_a+1)%perm[cycle].len() == vertex_b || (vertex_b+1)%perm[cycle].len() == vertex_a {
            delta+= 2.0*instance.dist_k(a, b);
        }
        Some(delta)
    }

    fn apply(&self, state: usize, solution: &mut TSPSolution) {
        let (cycle, vertex_a, vertex_b) = self.unpack_state(state, solution).unwrap();
        let perm = if cycle == 0 { &mut solution.perm_a } else { &mut solution.perm_b };
        let t = perm[vertex_a];
        perm[vertex_a] = perm[vertex_b];
        perm[vertex_b] = t;
    }

    // fn next(&mut self, instance: &TSPInstance, solution: &TSPSolution) -> Option<f32> {
    //     if self.cycle == 2 {
    //         return None;
    //     }
    //     let perm = vec![&solution.perm_a, &solution.perm_b];
    //     let (a_prev, a, a_next) = self.get_neighbors_in_cycle(self.vertex_a, perm[self.cycle]);
    //     let (b_prev, b, b_next) = self.get_neighbors_in_cycle(self.vertex_b, perm[self.cycle]);
    //     let delta = instance.dist_k(b_prev, a) + instance.dist_k(a, b_next)
    //         + instance.dist_k(a_prev, b) + instance.dist_k(b, a_next)
    //         - instance.dist_k(a_prev, a) - instance.dist_k(a, a_next)
    //         - instance.dist_k(b_prev, b) - instance.dist_k(b, b_next);
    //     self.prev_cycle = self.cycle;
    //     self.prev_vertex_a = self.vertex_a;
    //     self.prev_vertex_b = self.vertex_b;
    //     self.vertex_b += 1;
    //     if self.vertex_b == perm[self.cycle].len() {
    //         self.vertex_a += 1;
    //         self.vertex_b = self.vertex_a+1;
    //         if self.vertex_a+1 == perm[self.cycle].len() {
    //             self.cycle += 1;
    //             self.vertex_a = 0;
    //             self.vertex_b = 1;
    //         }
    //     }
    //     return Some(delta)
    // }
    // fn random(&mut self, instance: &TSPInstance, solution: &TSPSolution) -> Option<f32> {
    //     let mut rng = rand::thread_rng();
    //     self.cycle = rng.gen_range(0..2);
    //     let n = if self.cycle == 0 {solution.perm_a.len()} else {solution.perm_b.len()};
    //     self.vertex_a = rng.gen_range(0..n);
    //     self.vertex_b = rng.gen_range(0..n-1);
    //     if self.vertex_b >= self.vertex_a {
    //         self.vertex_b+=1;
    //     }
    //     self.next(instance, solution)
    // }
    // fn apply_last(&self, instance: &TSPInstance, solution: &mut TSPSolution) {
        
    // }
    // fn reset(&mut self) {
    //     self.cycle = 0;
    //     self.vertex_a = 0;
    //     self.vertex_b = 1;
    //     self.prev_cycle = 0;
    //     self.prev_vertex_a = 0;
    //     self.prev_vertex_b = 0;
    // }
}