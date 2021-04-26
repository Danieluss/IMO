use crate::traits::Solver;
use crate::tsp::def::{TSPInstance, TSPSolution};
use crate::tsp::partial_path::PartialPath;
use crate::tsp::picker::Picker;
use rand::Rng;



pub struct GreedySolver {
    picker: Box<dyn Picker>,
}

impl GreedySolver {
    pub fn new(picker: Box<dyn Picker>) -> GreedySolver {
        GreedySolver {
            picker
        }
    }

    fn remote_random(instance: &TSPInstance) -> (f32, usize, usize) {
        let mut max: (f32, usize, usize) = (-1., 0, 0);
        let n: usize = instance.dimension;
        let mut rng = rand::thread_rng();
        let i = rng.gen_range(0..n);
        for j in i + 1..n {
            let dist = instance.dist_k(i, j);
            if dist > max.0 {
                max = (dist, i, j);
            }
        }
        max
    }

    fn remote(start_vertex: usize, instance: &TSPInstance) -> (f32, usize, usize) {
        let mut max: (f32, usize, usize) = (-1., 0, 0);
        let n: usize = instance.dimension;
        for j in 0..n {
            if j == start_vertex {
                continue;
            }
            let dist = instance.dist_k(start_vertex, j);
            if dist > max.0 {
                max = (dist, start_vertex, j);
            }
        }
        max
    }
}

impl Solver<TSPInstance, TSPSolution> for GreedySolver{
    fn solve(&self, start_vertex: usize, instance: &TSPInstance, ) -> TSPSolution {
        let max = GreedySolver::remote(start_vertex, instance);
        let n: usize = instance.dimension;
        let mut visited = vec![false; n];
        visited[max.1] = true;
        visited[max.2] = true;
        let mut partial_a = PartialPath {
            instance: &instance,
            vec: vec![max.1],
        };
        let mut partial_b = PartialPath {
            instance: &instance,
            vec: vec![max.2],
        };

        // TODO: Może jakieś clusterowanie (możliwie najlepiej dzielimy na dwa równe podzbiory, a dopiero potem w ich obrębie wyznaczamy najlepsze cykle)
        while partial_a.vec.len() + partial_b.vec.len() < n {
            self.picker.add_both(&mut partial_a, &mut partial_b, &mut visited);
        }

        TSPSolution::new(
            partial_a.vec,
            partial_b.vec,
        )
    }
}