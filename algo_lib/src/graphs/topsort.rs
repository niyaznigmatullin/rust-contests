pub struct TopSortGraph {
    n: usize,
    edges: Vec<Vec<usize>>,
}

impl TopSortGraph {
    pub fn new(n: usize, edges_list: &Vec<(usize, usize)>) -> Self {
        let mut edges = vec![Vec::new(); n];
        for &(from, to) in edges_list {
            edges[from].push(to);
        }
        Self {
            n,
            edges,
        }
    }

    pub fn get_top_sort(&self) -> Vec<usize> {
        let mut top_sort = Vec::new();
        let mut used = vec![false; self.n];
        for v in 0..self.n {
            if !used[v] {
                self.dfs(v, &mut used, &mut top_sort);
            }
        }
        top_sort.reverse();
        top_sort
    }

    fn dfs(&self, v: usize, used: &mut Vec<bool>, top_sort: &mut Vec<usize>) {
        used[v] = true;
        for &to in &self.edges[v] {
            if !used[to] {
                self.dfs(to, used, top_sort);
            }
        }
        top_sort.push(v);
    }

    pub fn edges(&self, from: usize) -> &Vec<usize> {
        &self.edges[from]
    }
}
