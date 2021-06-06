use crate::tsp::def::TSPSolution;
use crate::tsp::def::TSPInstance;
use crate::tsp::neighborhoods::transition::Transition;

pub struct EdgesTransition {}

impl EdgesTransition {
    pub fn new() -> EdgesTransition {
        EdgesTransition {}
    }
    pub fn unpack_state(&self, state: usize, solution: &TSPSolution) -> Option<(usize, usize, usize)> {
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

    fn pack_state(&self, cycle: usize, vertex_a: usize, vertex_b: usize, solution: &TSPSolution) -> usize {
        // println!("{} {} {}", cycle, vertex_a, vertex_b);
        let mut vertex_a = vertex_a;
        let mut shift = (vertex_b as i32)-(vertex_a as i32)-2;
        let (n_a, n_b) = (solution.perm_a.len(), solution.perm_b.len());
        let mut state: usize;
        if cycle == 0 {
            vertex_a = (vertex_a+1)%n_a;
            if shift < 0 {
                shift+=n_a as i32;
            }
            shift = (shift%n_a as i32 + n_a as i32)%n_a as i32;
            state = (n_a-3)*vertex_a + (shift as usize);
        } else {
            vertex_a = (vertex_a+1)%n_b;
            if shift < 0 {
                shift+=n_b as i32;
            }
            shift = (shift%n_b as i32 + n_b as i32)%n_b as i32;
            state = (n_a-3)*n_a + (n_b-3)*vertex_a + (shift as usize);
        }
        state+=1;
        state
    }

    pub fn apply_explicit(&self, cycle: usize, vertex_a: usize, vertex_b: usize, solution: &mut TSPSolution) {
        self.apply(self.pack_state(cycle, vertex_a, vertex_b, solution), solution);
    }

    pub fn score_explicit(&self, cycle: usize, vertex_a: usize, vertex_b: usize, instance: &TSPInstance, solution: &TSPSolution) -> Option<f32> {
        self.score(self.pack_state(cycle, vertex_a, vertex_b, solution), instance, solution)
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
        let vertex_b = (vertex+shift+1)%perm[cycle].len();
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
        let mut b = (a+shift+1)%perm.len();
        if b < a {
            b+=1;
            a-=1;
            let t = b; b = a; a = t;
        }
        while a < b {
            let vertex_a = perm[a];
            let vertex_b = perm[b];
            let tmp = solution.order[vertex_b];
            solution.order[vertex_b] = solution.order[vertex_a];
            solution.order[vertex_a] = tmp;
            if solution.order[vertex_a] != b || solution.order[vertex_b] != a {
                println!("X");
            }
            assert_eq!(solution.order[vertex_a], b);
            assert_eq!(solution.order[vertex_b], a);
            let t = perm[a]; perm[a] = perm[b]; perm[b] = t;
            a+=1;
            b-=1;
        }
    }
    fn show_state(&self, state: usize, solution: &TSPSolution) {
        println!("{:?}", self.unpack_state(state, solution));
    }
}