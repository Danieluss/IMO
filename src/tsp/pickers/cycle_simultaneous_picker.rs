use crate::tsp::picker::Picker;
use crate::tsp::partial_path::PartialPath;

pub struct CycleSimultaneousPicker;

impl Picker for CycleSimultaneousPicker {
    fn add_both(&self, partial_path_a: &mut PartialPath, partial_path_b: &mut PartialPath, visited: &mut Vec<bool>) {
        let cost_a = self.get_costs(partial_path_a, visited);
        let cost_b = self.get_costs(partial_path_b, visited);
        let n = visited.len();
        let mut min_pair = (f32::MAX, 0, 0);
        for i in 0..n {
            if !visited[i] {
                for j in 0..n {
                    if visited[j] || i == j {
                        continue;
                    }
                    if cost_a[i].0 + cost_b[j].0 < min_pair.0 {
                        min_pair = (cost_a[i].0 + cost_b[j].0, i, j);
                    }
                }
            }
        }
        visited[min_pair.1] = true;
        visited[min_pair.2] = true;
        partial_path_a.vec.insert(cost_a[min_pair.1].1, min_pair.1);
        partial_path_b.vec.insert(cost_b[min_pair.2].1, min_pair.2);
    }
}

impl CycleSimultaneousPicker {
    pub fn get_costs(&self, partial_path: &mut PartialPath, visited: &mut Vec<bool>) -> Vec<(f32, usize)> {
        let n = partial_path.instance.dimension;
        let mut min_increase: Vec<(f32, usize)> = Vec::new();
        for j in 0..n {
            if !visited[j] {
                let mut min = (f32::MAX, 0);
                for i in 0..partial_path.vec.len() {
                    let new_score = partial_path.try_insert(i, j);
                    if new_score < min.0 {
                        min = (new_score, i);
                    }
                }
                min_increase.push(min);
            } else {
                min_increase.push((f32::MAX, 0));
            }
        }
        min_increase
    }
}