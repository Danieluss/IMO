use std::{fs};

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