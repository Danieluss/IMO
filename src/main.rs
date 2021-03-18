use std::env;
use std::time::Instant;
use std::fs::File;
use json;

use imo::utils::{Stat, contents, print_table_to_file, print_graph_to_file};
use imo::traits::Instance;
use imo::tsp::def::{TSPInstance, TSPSolution};
use imo::tsp::solvers_factory::SolversFactory;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("Usage {} config_path", args[0]);
    }
    let config = json::parse(&contents(&args[1])).unwrap();

    let mut scores = vec![vec![Stat::new(); config["instances"].len()]; config["algorithms"].len()];
    let mut times = vec![vec![Stat::new(); config["instances"].len()]; config["algorithms"].len()];
    let mut best_solutions = vec![vec![TSPSolution{perm_a: Vec::new(), perm_b: Vec::new()}; config["instances"].len()]; config["algorithms"].len()];

    let filepath = format!("res/{}", config["plots"].as_str().unwrap());
    let mut plot_file = File::create(&filepath).unwrap();
    let time_multiplier = config["time_multiplier"].as_f32().unwrap();

    for (j, instancename) in config["instances"].members().enumerate() {
        let filepath = format!("data/{}", &instancename.as_str().unwrap());
        let instance = TSPInstance::parse_file(&filepath);
        for (i, algorithm) in config["algorithms"].members().enumerate() {
            let solver = SolversFactory::create_from_json(algorithm);
            for k in 0..config["iterations"].as_usize().unwrap() {
                let start = Instant::now();
                let solution = solver.solve(k, &instance);
                let duration = start.elapsed();
                let score = instance.eval(&solution);
                let is_best = scores[i][j].update(score);
                times[i][j].update(duration.as_secs_f32()*time_multiplier);
                if is_best {
                    best_solutions[i][j] = solution;
                }
            }
            print_graph_to_file(&mut plot_file, algorithm["name"].as_str().unwrap(),
                config["plots_scale"].as_f32().unwrap(), &instance, &best_solutions[i][j]);
        }
    }

    let filepath = format!("res/{}", config["table"].as_str().unwrap());
    let mut file = File::create(&filepath).unwrap();

    print_table_to_file(&mut file, &scores, "avg", &config);
    print_table_to_file(&mut file, &scores, "min", &config);
    print_table_to_file(&mut file, &scores, "max", &config);

    print_table_to_file(&mut file, &times, "avg", &config);
    print_table_to_file(&mut file, &times, "min", &config);
    print_table_to_file(&mut file, &times, "max", &config);


}