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
        } else if value > self.max {
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
    for instancename in config["instances"].members() {
        write!(file, " & {}", instancename);
    }
    write!(file, "\\\\ \\hline\n");

    for (i, algorithm) in config["algorithms"].members().enumerate() {
        write!(file, "{} ", algorithm["name"].as_str().unwrap());
        
        for j in 0..config["instances"].len() {
            write!(file, "& {} ", stats[i][j].get(stat_name));
        }
        write!(file, "\\\\ \\hline\n");
    }

    write!(file, "\n\n");
}