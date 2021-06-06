use crate::tsp::similarity::Similarity;
use crate::tsp::def::{TSPSolution, TSPInstance};
use std::cmp::max;
use crate::tsp::similarities::vertex_similarity::VertexSimilarity;

pub struct EdgeSimilarity;

impl EdgeSimilarity {
    pub fn new() -> EdgeSimilarity {
        EdgeSimilarity {}
    }

    fn _sim(&self, solution_a: &TSPSolution, solution_b: &TSPSolution, reverse: bool) -> usize {
        let mut a_cycle_dest = 0;
        let mut a_perm_a = &solution_a.perm_a;
        let mut a_perm_b = &solution_a.perm_b;

        let b_perm_a = &solution_b.perm_a;
        let b_perm_b = &solution_b.perm_b;
        let b_order = &solution_b.order;
        let b_cycle = &solution_b.cycle;

        if reverse {
            a_perm_a = &solution_a.perm_b;
            a_perm_b = &solution_a.perm_a;
            a_cycle_dest = 1;
        }

        let mut similarity: usize = 0;
        let n = a_perm_a.len();

        for a_vert_i in 0..n {
            let g_vert = a_perm_a[a_vert_i];
            if b_cycle[g_vert] == a_cycle_dest
                && (b_perm_a[(b_order[g_vert] + 1) % n] == a_perm_a[(a_vert_i + 1) % n]
                || b_perm_a[(b_order[g_vert] + n - 1) % n] == a_perm_a[(a_vert_i + 1) % n]) {
                similarity += 1;
            }
        }

        for a_vert_i in 0..n {
            let g_vert = a_perm_b[a_vert_i];
            if b_cycle[g_vert] == 1 - a_cycle_dest
                && (b_perm_b[(b_order[g_vert] + 1) % n] == a_perm_b[(a_vert_i + 1) % n]
                || b_perm_b[(b_order[g_vert] + n - 1) % n] == a_perm_b[(a_vert_i + 1) % n]) {
                similarity += 1;
            }
        }

        similarity
    }
}

impl Similarity for EdgeSimilarity {
    fn sim(&self, instance: &TSPInstance, solution_a: &TSPSolution, solution_b: &TSPSolution) -> usize {
        return max(self._sim(solution_a, solution_b, true), self._sim(solution_a, solution_b, false));
    }
}