use crate::tsp::def::TSPSolution;
use crate::tsp::def::TSPInstance;
use crate::traits::Solver;
use std::collections::HashMap;
use crate::tsp::picker::Picker;
use crate::tsp::pickers::regret_picker::RegretPicker;
use crate::tsp::pickers::cycle_simultaneous_picker::CycleSimultaneousPicker;
use crate::tsp::pickers::cycle_picker::CyclePicker;
use crate::tsp::pickers::nearest_picker::NearestPicker;
use json;
use crate::tsp::random_solver::RandomSolver;
use crate::tsp::solver::GreedySolver;

pub struct SolversFactory;

impl SolversFactory {
    pub fn create_from_json(config: &json::JsonValue) -> Box<dyn Solver<TSPInstance, TSPSolution>> {
        if config["solver"] == "Random" {
            Box::new(RandomSolver)
        } else if config["solver"] == "Greedy" {
            let mut pickers: HashMap<&str, Box<dyn Picker>> = HashMap::new();
            pickers.insert("Nearest", Box::new(NearestPicker));
            pickers.insert("Cycle", Box::new(CyclePicker));
            pickers.insert("CycleSimultaneous", Box::new(CycleSimultaneousPicker));
            pickers.insert("Regret", Box::new(RegretPicker));
            let picker = pickers.remove(config["picker"].as_str().unwrap()).unwrap();
            Box::new(GreedySolver::new(picker))
        } else {
            Box::new(RandomSolver)
        }
    }
}