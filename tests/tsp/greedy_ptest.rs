
#[cfg(test)]
mod tests {
    use imo::tsp::def::TSPInstance;
    use imo::traits::{Instance, Solver};
    use imo::tsp::solver::GreedySolver;
    use imo::tsp::pickers::nearest_picker::NearestPicker;

    #[test]
    fn greedy_test() {
        let instance = TSPInstance::parse_file("data/kroA100.tsp");
        let solver = GreedySolver::new(Box::new(NearestPicker));
        let solution = solver.solve(0, &instance);
        println!("{}", instance.eval(&solution));
    }
}