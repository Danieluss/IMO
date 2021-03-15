use crate::tsp::def::TSPInstance;

pub struct PartialPath<'a> {
    pub instance: &'a TSPInstance,
    pub vec: Vec<usize>,
}

impl PartialPath<'_> {
    pub fn try_insert(&self, pos: usize, id: usize) -> f32 {
        let n = self.vec.len() as i32;
        let prev = ((((pos as i32) - 1) % n + n) % n) as usize;
        let next = pos;
        // println!("{} {} {} {}", prev, next, id, pos);
        - self.instance.dist_k(self.vec[prev], self.vec[next])
            + self.instance.dist_k(self.vec[prev], id) + self.instance.dist_k(id, self.vec[next])
    }
}