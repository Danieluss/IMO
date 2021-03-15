
#[cfg(test)]
mod tests {
    use imo::tsp::def::TSPInstance;
    use imo::traits::{Instance, Solver};
    use imo::tsp::random_solver::RandomSolver;
    use imo::tsp::solver::GreedySolver;
    use imo::tsp::picker::Picker;
    use imo::tsp::pickers::nearest_picker::NearestPicker;
    use imo::tsp::pickers::cycle_picker::CyclePicker;
    use imo::tsp::pickers::regret_picker::RegretPicker;
    use imo::tsp::pickers::cycle_simultaneous_picker::CycleSimultaneousPicker;

    fn check_all_nodes(perm_a: &Vec<usize>, perm_b: &Vec<usize>, n: usize) {
        let mut v: Vec<usize> = Vec::new();
        assert_eq!(perm_a.len(), perm_b.len());
        v.append(&mut perm_a.clone());
        v.append(&mut perm_b.clone());
        v.sort();
        let mut j = 0;
        for i in 0..n {
            assert_eq!(v[j], i);
            j+=1;
        }
    }

    #[test]
    fn nearest_greedy_test() {
        let instance = TSPInstance::parse_file("data/kroA100.tsp");
        let solver = GreedySolver::new(NearestPicker::add_both);
        let solution = solver.solve(0, &instance);
        println!("Nearest: {}", instance.eval(&solution));
        check_all_nodes(&solution.perm_a, &solution.perm_b, 100);
    }

    #[test]
    fn cycle_greedy_test() {
        let instance = TSPInstance::parse_file("data/kroA100.tsp");
        let solver = GreedySolver::new(CyclePicker::add_both);
        let solution = solver.solve(0, &instance);
        println!("Cycle: {}", instance.eval(&solution));
        check_all_nodes(&solution.perm_a, &solution.perm_b, 100);
    }

    #[test]
    fn cycle_simultaneous_greedy_test() {
        let instance = TSPInstance::parse_file("data/kroA100.tsp");
        let solver = GreedySolver::new(CycleSimultaneousPicker::add_both);
        let solution = solver.solve(0, &instance);
        println!("Cycle: {}", instance.eval(&solution));
        check_all_nodes(&solution.perm_a, &solution.perm_b, 100);
    }

    #[test]
    fn regret_greedy_test() {
        let instance = TSPInstance::parse_file("data/kroA100.tsp");
        let solver = GreedySolver::new(RegretPicker::add_both);
        let solution = solver.solve(0, &instance);
        println!("Regret: {}", instance.eval(&solution));
        check_all_nodes(&solution.perm_a, &solution.perm_b, 100);
    }

    #[test]
    fn random_test() {
        let instance = TSPInstance::parse_file("data/kroA100.tsp");
        let solver = RandomSolver::new();
        let solution = solver.solve(0, &instance);
        println!("{}", instance.eval(&solution));
        check_all_nodes(&solution.perm_a, &solution.perm_b, 100);
    }
}