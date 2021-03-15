use std::env;
use json;

mod utils;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("Usage {} config_path", args[0]);
    }
    let config = json::parse(&utils::contents(&args[1])).unwrap();
    println!("{:?}", config["files"]);
}