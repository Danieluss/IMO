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
    pub permutation: Vec<usize>
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
    fn calc_dist_matrix(&mut self) {
        for i in 0..self.dimension {
            self.distance_cache.push(Vec::new());
            for j in 0..self.dimension {
                let distance = ((self.cities[i].x - self.cities[j].x).powi(2) + (self.cities[i].y - self.cities[j].y).powi(2)).sqrt();
                self.distance_cache[i].push(distance)
            }
        }
    }
}

impl Instance<TSPSolution> for TSPInstance {
    fn eval(&self, solution: &TSPSolution) -> f32 {
        if self.dimension != solution.permutation.len() {
            panic!("Solution of inadequate size was given")
        }
        let permutation = &solution.permutation;
        let mut acc: f32 = 0.;
        for i in 0..self.dimension {
            acc += self.dist_k(permutation[i], permutation[(i + 1) % self.dimension]);
        }
        acc
    }

    fn random_solution(&self) -> TSPSolution {
        TSPSolution {
            permutation: random_permutation(self.dimension)
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

impl TSPInstance {
    fn dist_c(&self, a: &City, b: &City) -> f32 {
        self.distance_cache[a.id][b.id]
    }

    fn dist_k(&self, a: usize, b: usize) -> f32 {
        self.distance_cache[a][b]
    }
}

def_str_consts! {
    NAME, DIMENSION, NODE_COORD_SECTION
}
