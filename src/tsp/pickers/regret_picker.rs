use crate::tsp::picker::Picker;
use crate::tsp::partial_path::PartialPath;
use crate::tsp::pickers::cycle_picker::CyclePicker;

pub struct RegretPicker;

impl Picker for RegretPicker {
    fn add_both(&self, partial_path_a: &mut PartialPath, partial_path_b: &mut PartialPath, visited: &mut Vec<bool>) {
        while (partial_path_a.vec.len() < partial_path_b.vec.len()) {
            self.add(partial_path_a, partial_path_b, visited);
        }
        while (partial_path_a.vec.len() > partial_path_b.vec.len()) {
            self.add(partial_path_b, partial_path_a, visited);
        }
        self.add(partial_path_a, partial_path_b, visited);
        self.add(partial_path_b, partial_path_a, visited);
    }
}

impl RegretPicker {
    fn add(&self, partial_path: &mut PartialPath, other_partial_path: &PartialPath, visited: &mut Vec<bool>) {
        let n = partial_path.instance.dimension;
        if partial_path.vec.len() < 3 {
            let picker = CyclePicker::new();
            picker.add(partial_path, visited);
            return;
        }
        let mut max_regret = (f32::MIN, 0, 0);
        for j in 0..n {
            if !visited[j] {
                let mut min1 = (f32::MAX, 0);
                let mut min2 = f32::MAX;
                for i in 0..partial_path.vec.len() {
                    let new_score = partial_path.try_insert(i, j);
                    if new_score < min1.0 {
                        min2 = min1.0;
                        min1 = (new_score, i);
                    } else if new_score < min2 {
                        min2 = new_score;
                    }
                }
                for i in 0..other_partial_path.vec.len() { //check if it's better to put this vertex on the second path
                    let new_score = other_partial_path.try_insert(i, j);
                    if new_score < min2 {
                        min2 = new_score;
                    }
                }
                if min2 - min1.0 > max_regret.0 {
                    max_regret = (min2-min1.0, min1.1, j);
                }
            }
        }
        visited[max_regret.2] = true;
        partial_path.vec.insert(max_regret.1, max_regret.2);
    }
}