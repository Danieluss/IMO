use crate::tsp::def::TSPSolution;
use crate::tsp::def::TSPInstance;
use crate::tsp::neighborhoods::transition::Transition;

pub struct EdgesTransition {}

impl EdgesTransition {
    pub fn new() -> EdgesTransition {
        EdgesTransition {}
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
}