use rand::Rng;

use crate::tsp::def::TSPSolution;
use crate::tsp::def::TSPInstance;
use crate::tsp::neighborhoods::transition::Transition;
use crate::primes::primes::Primes;

pub struct Neighborhood {
    generator: usize,
    group_size: usize,
    random: bool,
    state: usize,
    active: bool,
    start_state: usize,    
    transitions: Vec<Box<dyn Transition>>,
    transition_sizes: Vec<usize>
}

impl Neighborhood {
    
    pub fn new(transitions: Vec<Box<dyn Transition>>, solution: &TSPSolution, random: bool) -> Neighborhood{
        let mut transition_sizes = Vec::new();
        let mut neighborhood_size: usize = 0;
        for transition in &transitions {
            transition_sizes.push(transition.size(solution));
            neighborhood_size+=transition_sizes.last().unwrap();
        }
        let (generator, group_size) = if random {
            Primes::group_generator_and_size(neighborhood_size)
        } else {
            (1, neighborhood_size+1)
        };
        let mut neighborhood = Neighborhood {
            generator,
            group_size,
            random,
            state: 0,
            active: false,
            start_state: 0,    
            transitions,
            transition_sizes
        };
        neighborhood.reset();
        neighborhood
    }

    pub fn next(&mut self, instance: &TSPInstance, solution: &TSPSolution) -> Option<(f32, usize)> {
        loop {
            if self.active && self.state == self.start_state {
                return None;
            }
            self.active = true;
            let mut score: Option<f32> = None;
            let mut current_state = self.state;
            for i in 0..self.transitions.len() {
                if current_state <= self.transition_sizes[i] {
                    score = self.transitions[i].score(current_state, &instance, &solution);
                    break;
                }
                current_state-=self.transition_sizes[i];
            }
            let prev_state = self.state;
            if self.random {
                self.state = (self.state*self.generator)%self.group_size;
            } else {
                self.state = (self.state+1)%self.group_size;
                if self.state == 0 {
                    self.state+=1;
                }
            }
            match score {
                Some(x) => return Some((x, prev_state)),
                None => {}
            }
        }
    }
    
    pub fn apply_transition(&self, transition: usize, solution: &mut TSPSolution) {
        let mut current_state = transition;
        for i in 0..self.transitions.len() {
            if current_state <= self.transition_sizes[i] {
                self.transitions[i].apply(current_state, solution);
                break;
            }
            current_state-=self.transition_sizes[i];
        }
    }

    pub fn show_transition(&self, transition: usize, solution: &TSPSolution) {
        let mut current_state = transition;
        for i in 0..self.transitions.len() {
            if current_state <= self.transition_sizes[i] {
                self.transitions[i].show_state(current_state, solution);
                break;
            }
            current_state-=self.transition_sizes[i];
        }
    }

    pub fn reset(&mut self) {
        if self.random {
            let mut rng = rand::thread_rng();
            self.state = rng.gen_range(1..self.group_size);
        } else {
            self.state = 1;
        }
        self.start_state = self.state;
        self.active = false;
    }
}