use crate::tsp::neighborhoods::transition::Transition;
use crate::tsp::def::TSPSolution;
use crate::tsp::def::TSPInstance;

pub struct VertexTransition {}

impl VertexTransition {
    pub fn new() -> VertexTransition {
        VertexTransition {}
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
}