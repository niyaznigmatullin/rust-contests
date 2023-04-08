
pub struct UnionFind {
    pub p: Vec<usize>,
}

impl UnionFind {
    pub fn new(n: usize) -> Self {
        Self {
            p: (0..n).collect(),
        }
    }

    pub fn find(&mut self, v: usize) -> usize {
        if self.p[v] != v {
            self.p[v] = self.find(self.p[v]);
        }
        self.p[v]
    }

    pub fn join(&mut self, mut v: usize, mut u: usize) -> bool {
        v = self.find(v);
        u = self.find(u);
        self.p[v] = u;
        v != u
    }
}
