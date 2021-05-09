pub trait Solution {}

pub trait Instance<O> where O: Solution {
    fn eval(&self, solution: &O) -> f32;

    fn parse_file(file_name: &str) -> Self;
}

pub trait MetaInstance<O>: Instance<O> where O: Solution {
    fn neighbours(solution: &O) -> Vec<O>;
}

pub trait Solver<I, O> where I: Instance<O>, O: Solution {
    fn solve(&self, start_vertex: usize, instance: &I) -> O;
    fn solve_s(&self, start_vertex: usize, instance: &I, solution: O) -> O;
}
