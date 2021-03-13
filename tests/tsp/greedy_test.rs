
#[cfg(test)]
mod tests {
    use imo::tsp::def::TSPInstance;
    use imo::traits::Instance;

    #[test]
    fn greedy_test() {
        let instance = TSPInstance::parse_file("data/kroA100.tsp");
        let solution = instance.random_solution();
        println!("{}", instance.eval(&solution));
    }
}