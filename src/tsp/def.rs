use crate::traits::Instance;
use crate::traits::Solution;
use crate::utils::contents;

#[derive(Debug)]
pub struct City {
    id: usize,
    label: String,
    x: f32,
    y: f32,
}

impl City {
    pub fn get_coord(&self) -> (f32, f32) {
        (self.x, self.y)
    }
}

pub struct TSPSolution {
    pub perm_a: Vec<usize>,
    pub perm_b: Vec<usize>,
    pub cycle: Vec<usize>,
    pub order: Vec<usize>,
}

impl TSPSolution {
    pub fn new(perm_a: Vec<usize>, perm_b: Vec<usize>) -> TSPSolution {
        let mut cycle = Vec::new();
        let mut order = Vec::new();
        for _ in 0..perm_a.len() + perm_b.len() {
            cycle.push(0);
            order.push(0);
        }
        for i in 0..perm_a.len() {
            let v = perm_a[i];
            cycle[v] = 0;
            order[v] = i;
        }
        for i in 0..perm_b.len() {
            let v = perm_b[i];
            cycle[v] = 1;
            order[v] = i;
        }
        TSPSolution {
            perm_a,
            perm_b,
            cycle,
            order,
        }
    }

    pub fn reorder(&mut self) {
        let mut i = 0;
        for val in self.perm_a.iter() {
            self.order[*val] = i;
            self.cycle[*val] = 0;
            i += 1;
        }
        let mut i = 0;
        for val in self.perm_b.iter() {
            self.order[*val] = i;
            self.cycle[*val] = 1;
            i += 1;
        }
    }

    pub fn check(&self) -> bool {
        let mut res = true;
        let mut i = 0;
        for val in self.perm_a.iter() {
            res &= (self.order[*val] == i);
            res &= (self.cycle[*val] == 0);
            i += 1;
        }
        let mut i = 0;
        for val in self.perm_b.iter() {
            res &= (self.order[*val] == i);
            res &= (self.cycle[*val] == 1);
            i += 1;
        }
        res
    }

    pub fn deep_clone(&self) -> Self {
        TSPSolution {
            perm_a: self.perm_a.clone(),
            perm_b: self.perm_b.clone(),
            cycle: self.cycle.clone(),
            order: self.order.clone(),
        }
    }
}

impl Solution for TSPSolution {}

impl Clone for TSPSolution {
    fn clone(&self) -> Self {
        TSPSolution {
            perm_a: self.perm_a.clone(),
            perm_b: self.perm_b.clone(),
            cycle: Vec::new(),
            order: Vec::new(),
        }
    }
}

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

    fn eval_permutation(&self, perm: &Vec<usize>) -> f32 {
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
