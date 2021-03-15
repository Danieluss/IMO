use crate::tsp::picker::Picker;
use crate::tsp::partial_path::PartialPath;

pub struct CyclePicker;

impl Picker for CyclePicker {
    fn add_both(partial_path_a: &mut PartialPath, partial_path_b: &mut PartialPath, visited: &mut Vec<bool>) {
        CyclePicker::add(partial_path_a, visited);
        CyclePicker::add(partial_path_b, visited);
    }
}

impl CyclePicker {
    pub fn add(partial_path: &mut PartialPath, visited: &mut Vec<bool>) {
        let n = partial_path.instance.dimension;
        let mut min = (f32::MAX, 0, 0);
        for i in 0..partial_path.vec.len() {
            for j in 0..n {
                if !visited[j] {
                    let new_score = partial_path.try_insert(i, j);
                    if min.0 > new_score {
                        min = (new_score, i, j);
                    }
                }
            }
        }
        visited[min.2] = true;
        partial_path.vec.insert(min.1, min.2);
    }
}