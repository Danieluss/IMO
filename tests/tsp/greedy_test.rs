
#[cfg(test)]
mod tests {
    use imo::tsp::def::TSPInstance;
    use imo::traits::{Instance, Solver};
    use imo::tsp::solver::GreedySolver;

    #[test]
    fn greedy_test() {
        let instance = TSPInstance::parse_file("data/kroA100.tsp");
        let solution = GreedySolver::solve(&instance);
        println!("{}", instance.eval(&solution));
    }

    #[test]
    fn random_test() {
        let instance = TSPInstance::parse_file("data/kroA100.tsp");
        let solution = instance.random_solution();
        println!("{}", instance.eval(&solution));
    }
}