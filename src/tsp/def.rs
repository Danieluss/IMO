use crate::traits::Instance;
use crate::traits::Solution;
use crate::utils::{contents, random_permutation};

#[derive(Debug)]
pub struct City {
    id: usize,
    label: String,
    x: f32,
    y: f32,
}

pub struct TSPSolution {
    pub perm_a: Vec<usize>,
    pub perm_b: Vec<usize>
}

impl Solution for TSPSolution {}

#[derive(Default, Debug)]
pub struct TSPInstance {
    pub name: String,
    pub dimension: usize,
    pub cities: Vec<City>,
    pub distance_cache: Vec<Vec<f32>>,
}

impl TSPInstance {
    pub fn dist_c(&self, a: &City, b: &City) -> f32 {
        self.distance_cache[a.id][b.id]
    }

    pub fn dist_k(&self, a: usize, b: usize) -> f32 {
        self.distance_cache[a][b]
    }

    pub fn calc_dist_matrix(&mut self) {
        for i in 0..self.dimension {
            self.distance_cache.push(Vec::new());
            for j in 0..self.dimension {
                let distance = ((self.cities[i].x - self.cities[j].x).powi(2)
                    + (self.cities[i].y - self.cities[j].y).powi(2))
                    .sqrt()
                    .round();
                self.distance_cache[i].push(distance)
            }
        }
    }

    fn eval_permutation(&self, perm: &Vec<usize>) -> f32{
        let mut acc: f32 = 0.;
        for i in 0..perm.len() {
            acc += self.dist_k(perm[i], perm[(i + 1) % perm.len()]);
        }
        acc
    }
}

impl Instance<TSPSolution> for TSPInstance {
    fn eval(&self, solution: &TSPSolution) -> f32 {
        if self.dimension != (solution.perm_a.len() + solution.perm_b.len()) {
            panic!("Solution of inadequate size was given")
        }
        self.eval_permutation(&solution.perm_a) +
            self.eval_permutation(&solution.perm_b)
    }

    fn random_solution(&self) -> TSPSolution {
        let perm = random_permutation(self.dimension);
        TSPSolution {
            perm_a: perm[..(perm.len() + 1)/2].to_vec(),
            perm_b: perm[(perm.len() + 1)/2..].to_vec()
        }
    }

    fn parse_file(file_name: &str) -> Self {
        let cts = contents(file_name);
        let mut instance = TSPInstance::default();
        let mut read_points = false;
        let mut i = 0;

        for line in cts.lines() {
            let line: String = line.trim().parse().unwrap();
            if line == "EOF" {
                break;
            }
            if !read_points {
                let split: Vec<&str> = line.split(":").collect::<Vec<&str>>();
                let (k, v) = if split.len() > 1 {
                    (split[0].trim(), split[1].trim())
                } else {
                    (split[0].trim(), split[0].trim())
                };
                match k {
                    NAME => {
                        instance.name = String::from(v);
                    }
                    DIMENSION => {
                        instance.dimension = String::from(v).parse().unwrap();
                    }
                    NODE_COORD_SECTION => {
                        read_points = true;
                    }
                    _ => ()
                }
            } else {
                let split: Vec<&str> = line.split(" ").collect::<Vec<&str>>();
                let city = City {
                    id: i,
                    label: String::from(split[0]),
                    x: split[1].parse::<f32>().unwrap(),
                    y: split[2].parse::<f32>().unwrap(),
                };
                i += 1;
                instance.cities.push(city)
            }
        }

        instance.calc_dist_matrix();
        instance
    }
}

def_str_consts! {
    NAME, DIMENSION, NODE_COORD_SECTION
}
