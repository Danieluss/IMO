use crate::tsp::def::TSPInstance;
use crate::tsp::def::TSPSolution;

pub trait Transition {
    fn size(&self, solution: &TSPSolution) -> usize;
    fn score(&self, state: usize, instance: &TSPInstance, solution: &TSPSolution) -> Option<f32>;
    fn apply(&self, state: usize, solution: &mut TSPSolution);
    fn get_neighbors_in_cycle(&self, id: usize, perm:  &Vec<usize>) -> (usize, usize, usize) {
        let n = perm.len();
        (perm[(id+n-1)%n], perm[id], perm[(id+1)%n])
    }
}