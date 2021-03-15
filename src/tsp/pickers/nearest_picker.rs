use crate::tsp::picker::Picker;
use crate::tsp::partial_path::PartialPath;

pub struct NearestPicker;

impl Picker for NearestPicker {
    fn add_both(partial_path_a: &mut PartialPath, partial_path_b: &mut PartialPath, visited: &mut Vec<bool>) {
        NearestPicker::add(partial_path_a, visited);
        NearestPicker::add(partial_path_b, visited);
    }
}

impl NearestPicker {
    fn add(partial_path: &mut PartialPath<'_>, visited: &mut std::vec::Vec<bool>) {
        let mut min = (f32::MAX, 0);
        let n = partial_path.instance.dimension;
        for i in 0..n {
            let distance = partial_path.instance.dist_k(*partial_path.vec.last().unwrap(), i);
            if !visited[i] && distance < min.0 {
                min = (distance, i);
            }
        }
        visited[min.1] = true;
        partial_path.vec.push(min.1);
    }
}