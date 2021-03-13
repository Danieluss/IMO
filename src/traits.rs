pub trait Solution {}

pub trait Instance<O> where O: Solution {
    fn eval(&self, solution: &O) -> f32;
    fn random_solution(&self) -> O;

    fn parse_file(file_name: &str) -> Self;
}

pub trait MetaInstance<O>: Instance<O> where O: Solution {
    fn neighbours(solution: &O) -> Vec<O>;
}

trait Solver<I, O> where I: Instance<O>, O: Solution {
    fn solve(&self, instance: &I) -> O;
}
