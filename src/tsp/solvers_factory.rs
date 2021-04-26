use crate::tsp::local_solvers::LocalRandomWalker;
use crate::tsp::neighborhoods::vertex_transition::VertexTransition;
use crate::tsp::neighborhoods::edges_transition::EdgesTransition;
use crate::tsp::neighborhoods::transition::Transition;
use crate::tsp::neighborhoods::inter_cycle_transition::InterCycleTransition;
use crate::tsp::local_solvers::LocalGreedySolver;
use crate::tsp::local_solvers::LocalSteepestSolver;
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
use crate::tsp::candidate_solver::CandidateSolver;

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
        } else if config["solver"] == "Local" {
            let mut transitions: HashMap<&str, fn() -> Vec<Box<dyn Transition>>> = HashMap::new();
            transitions.insert("Vertex", || {vec![Box::new(InterCycleTransition{}), Box::new(VertexTransition{})]});
            transitions.insert("Edges", || {vec![Box::new(InterCycleTransition{}), Box::new(EdgesTransition{})]});
            if config["type"] == "Greedy" {
                Box::new(LocalGreedySolver::new(
                    SolversFactory::create_from_json(&config["initial_solver"]),
                    transitions.remove(config["transition"].as_str().unwrap()).unwrap()
                ))
            } else if config["type"] == "Steepest" {
                Box::new(LocalSteepestSolver::new(
                    SolversFactory::create_from_json(&config["initial_solver"]),
                    transitions.remove(config["transition"].as_str().unwrap()).unwrap()
                ))
            } else {
                Box::new(LocalRandomWalker::new(
                    SolversFactory::create_from_json(&config["initial_solver"]),
                    transitions.remove(config["transition"].as_str().unwrap()).unwrap()
                ))
            }
        } else if config["solver"] == "Candidate" {
            Box::new(CandidateSolver::new(
                config["num_neighbors"].as_usize().unwrap(),
                SolversFactory::create_from_json(&config["initial_solver"])
            ))
        } else {
            Box::new(RandomSolver)
        }
    }
}