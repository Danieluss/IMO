use std::collections::HashMap;
use crate::tsp::partial_path::PartialPath;
use crate::tsp::picker::Picker;
use crate::tsp::pickers::regret_picker::RegretPicker;
use crate::tsp::pickers::cycle_simultaneous_picker::CycleSimultaneousPicker;
use crate::tsp::pickers::cycle_picker::CyclePicker;
use crate::tsp::pickers::nearest_picker::NearestPicker;
use json;
use crate::random_solver::RandomSolver;
use crate::solver::GreedySolver;

struct SolversFactory;

impl SolversFactory {
    fn create_from_json(config: json::JsonValue) -> Box<Solver> {
        if config["solver"] == "Random" {
            RandomSolver()
        } else if config["solver"] == "Greedy" {
            // let pickers: Vec<&str, Box<fn(&mut PartialPath, &mut PartialPath, &mut Vec<bool>)>> = vec![
            //     ("Nearest", NearestPicker::add_both),
            //     ("Cycle", CyclePicker::add_both),
            //     ("CycleSimultaneous", CycleSimultaneousPicker::add_both),
            //     ("Regret", RegretPicker::add_both)
            // ];
            // let pickers = vec![("aa", "bb"), ("ac", "ac")];
            // let pickers = pickers.iter().collect::<HashMap<&str, &str>>();
            let pickers: HashMap<&str, Box<Picker>> = HashMap::new();
            pickers.insert("aa", NearestPicker);
            pickers.insert("aa", CyclePicker);
            let picker = pickers[config["picker"].as_str().unwrap()];
            GreedySolver {
                picker::add_both
            }
        }
    }
}