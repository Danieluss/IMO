use crate::traits::Instance;
use crate::traits::Solution;
use crate::traits::Solver;
use crate::tsp::def::{TSPInstance, TSPSolution};
use std::collections::{LinkedList, HashSet};
use rand::Rng;

pub struct PartialPath<'a> {
    instance: &'a TSPInstance,
    vec: Vec<usize>,
    score: f32,
}

impl PartialPath<'_> {
    fn try_insert(&self, pos: usize, id: usize) -> f32 {
        // TODO: do naprawy, w sobotę się tym zajmę
        let prev = (((pos as i32) - 1) % (self.vec.len() as i32)) as usize;
        let next = pos;
        let mut tmp_score = self.score - self.instance.dist_k(self.vec[prev], self.vec[next])
            + self.instance.dist_k(self.vec[prev], id) + self.instance.dist_k(id, self.vec[next]);
        tmp_score
    }
}

pub struct GreedySolver;

impl GreedySolver {
    fn remote(instance: &TSPInstance) -> (f32, usize, usize) {
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

    fn add(partial_path: &mut PartialPath, visited: &mut Vec<bool>) {
        let mut choice = (f32::MAX, 0);
        let n: usize = partial_path.instance.dimension;
        for i in 0..partial_path.vec.len() {
            for j in 0..n {
                let dist = partial_path.instance.dist_k(i, j);
                if !visited[j] && dist < choice.0 {
                    choice = (dist, j)
                }
            }
        }
        visited[choice.1] = true;
        let closest_id = choice.1;
        choice = (f32::MAX, 0);
        for i in 0..partial_path.vec.len() {
            let new_score = partial_path.try_insert(i, closest_id);
            if choice.0 > new_score {
                choice = (new_score, i);
            }
        }
        partial_path.vec.insert(choice.1, closest_id);
    }
}

impl Solver<TSPInstance, TSPSolution> for GreedySolver {
    fn solve(instance: &TSPInstance) -> TSPSolution {
        let max = GreedySolver::remote(instance);
        let n: usize = instance.dimension;
        let mut visited = vec![false; n];
        visited[max.1] = true;
        visited[max.2] = true;
        let mut partial_a = PartialPath {
            instance: &instance,
            vec: vec![max.1],
            score: 0.0,
        };
        let mut partial_b = PartialPath {
            instance: &instance,
            vec: vec![max.2],
            score: 0.0,
        };

        // TODO: warto zrobić wersję, że wybieramy dwa wierzchołki na raz (minimalna sumaryczna odległość do powstających cykli)?
        // TODO: Może jakieś clusterowanie (możliwie najlepiej dzielimy na dwa równe podzbiory, a dopiero potem w ich obrębie wyznaczamy najlepsze cykle)
        while partial_a.vec.len() + partial_b.vec.len() < n {
            GreedySolver::add(&mut partial_a, &mut visited);
            GreedySolver::add(&mut partial_b, &mut visited);
        }

        TSPSolution {
            perm_a: partial_a.vec,
            perm_b: partial_b.vec,
        }
    }
}