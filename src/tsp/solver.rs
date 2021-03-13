use crate::traits::Instance;
use crate::traits::Solution;
use crate::traits::Solver;
use crate::tsp::def::{TSPInstance, TSPSolution};
use std::collections::{LinkedList, HashSet};
use rand::Rng;

pub struct GreedySolver;

impl GreedySolver {
    fn remote(instance: &TSPInstance) -> (f32, usize, usize){
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

    fn add(instance: &TSPInstance, visited: &mut Vec<bool>, vec: &mut Vec<usize>) {
        let mut choice = (f32::MAX, 0);
        let n: usize = instance.dimension;
        for i in 0..vec.len() {
            for j in 0..n {
                let dist = instance.dist_k(i, j);
                if !visited[j] && dist < choice.0 {
                    choice = (dist, j)
                }
            }
        }
        visited[choice.1] = true;
        vec.push(choice.1);
    }
}

impl Solver<TSPInstance, TSPSolution> for GreedySolver {
    fn solve(instance: &TSPInstance) -> TSPSolution {
        let max = GreedySolver::remote(instance);
        let n: usize = instance.dimension;
        let mut visited = vec![false; n];
        visited[max.1] = true;
        visited[max.2] = true;
        let mut vec_a: Vec<usize> = vec![max.1];
        let mut vec_b: Vec<usize> = vec![max.2];

        // TODO: warto zrobić wersję, że wybieramy dwa wierzchołki na raz (minimalna sumaryczna odległość do powstających cykli)?
        // TODO: Może jakieś clusterowanie (możliwie najlepiej dzielimy na dwa równe podzbiory, a dopiero potem w ich obrębie wyznaczamy najlepsze cykle)
        while vec_a.len() + vec_b.len() < n {
            GreedySolver::add(instance, &mut visited, &mut vec_a);
            GreedySolver::add(instance, &mut visited, &mut vec_b);
        }

        TSPSolution {
            perm_a: vec_a,
            perm_b: vec_b
        }
    }
}