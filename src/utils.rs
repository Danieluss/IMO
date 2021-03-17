use crate::tsp::def::TSPSolution;
use crate::tsp::def::TSPInstance;
use std::{fs};
use std::fs::File;
use std::io::prelude::*;
use json;

use rand::Rng;


pub fn random_permutation(n: usize) -> Vec<usize> {
    let mut rng = rand::thread_rng();
    let mut vec = Vec::with_capacity(n);
    for i in 0..n {
        vec.push(i);
    }
    for i in 0..n - 2 {
        let j = i + rng.gen_range(0..n - i);
        vec.swap(i, j);
    }
    vec
}

pub fn contents(file_name: &str) -> String {
    fs::read_to_string(file_name).expect("Something went wrong")
}

#[derive(Debug)]
pub struct Stat {
    min: f32,
    max: f32,
    sum: f32,
    count: usize
}

impl Stat {
    pub fn new() -> Stat{
        Stat {
            min: f32::MAX,
            max: 0.0,
            sum: 0.0,
            count: 0
        }
    }
    pub fn update(&mut self, value: f32) -> bool {
        let mut flag = false;
        if value < self.min {
            self.min = value;
            flag = true;
        }
        if value > self.max {
            self.max = value;
        }
        self.sum+= value;
        self.count+=1;
        flag
    }
    fn get_max(&self) -> f32 {
        self.max
    }
    fn get_min(&self) -> f32 {
        self.min
    }
    fn get_avg(&self) -> f32 {
        self.sum/(self.count as f32)
    }
    fn get(&self, name: &str) -> f32 {
        if name == "min" {
            self.get_min()
        } else if name == "max" {
            self.get_max()
        } else {
            self.get_avg()
        }
    }
}

impl Clone for Stat {
    fn clone(&self) -> Self {
        Stat {
            min: self.min,
            max: self.max,
            sum: self.sum,
            count: self.count
        }
    }
}

pub fn print_table_to_file(file: &mut File, stats: &Vec<Vec<Stat>>, stat_name: &str, config: &json::JsonValue) {
    write!(file, "\\begin{{table}}[H]
    \\centering
    \\begin{{tabular}}{{|l|");
    for _ in 0..config["instances"].len() {
        write!(file, "r|");
    }
    write!(file, "}}
    \\hline
    Algorithm");

    for instancename in config["instances"].members() {
        write!(file, " & {}", instancename);
    }
    write!(file, "\\\\ \\hline\n");

    let mut min_id = vec![0; config["instances"].len()];

    for i in 0..config["algorithms"].len() {
        for j in 0..config["instances"].len() {
            if stats[i][j].get(stat_name) < stats[min_id[j]][j].get(stat_name) {
                min_id[j] = i;
            }
        }
    }

    for (i, algorithm) in config["algorithms"].members().enumerate() {
        write!(file, "{} ", algorithm["name"].as_str().unwrap());
        
        for j in 0..config["instances"].len() {
            write!(file, " & ");
            if min_id[j] == i {
                write!(file, "\\textbf{{");
            }
            write!(file, "{}", stats[i][j].get(stat_name));
            if min_id[j] == i {
                write!(file, "}}");
            }
        }
        write!(file, "\\\\ \\hline\n");
    }

    write!(file, "\\end{{tabular}}
    \\caption{{{}}}
\\end{{table}}", stat_name);

    write!(file, "\n\n");
}

fn print_path(file: &mut File, color: &str, scale: f32, instance: &TSPInstance, perm: &Vec<usize>) {
    for i in 0..perm.len() {
        let city_a_coord = instance.cities[perm[i]].get_coord();
        let city_b_coord = instance.cities[perm[(i+1)%perm.len()]].get_coord();
        write!(file, "\\draw[color={}, thick] ({},{}) -- ({},{});\n", color, city_a_coord.0*scale, city_a_coord.1*scale, city_b_coord.0*scale, city_b_coord.1*scale);
    }
}

pub fn print_graph_to_file(file: &mut File, name: &str, scale: f32, instance: &TSPInstance, solution: &TSPSolution) {
    write!(file, "\\begin{{subfigure}}[b]{{0.45\\textwidth}}
    \\centering
    \\begin{{tikzpicture}}");

    print_path(file, "red", scale, instance, &solution.perm_a);
    print_path(file, "blue", scale, instance, &solution.perm_b);

    write!(file, "\\end{{tikzpicture}}
    \\caption{{{}}}
    \\end{{subfigure}}\n", name);
}