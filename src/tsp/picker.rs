use crate::tsp::partial_path::PartialPath;

pub trait Picker {
    fn add_both(&self, partial_path_a: &mut PartialPath, partial_path_b: &mut PartialPath, visited: &mut Vec<bool>);
}