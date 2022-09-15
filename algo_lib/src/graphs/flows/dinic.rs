use std::cmp::min;
use std::collections::VecDeque;

pub struct Edge {
    from: usize,
    to: usize,
    cap: i64,
    flow: i64,
}

pub struct DinicGraph {
    edges: Vec<Edge>,
    adjacent: Vec<Vec<usize>>,
}

impl DinicGraph {
    pub fn new(n: usize) -> Self {
        Self {
            edges: Vec::new(),
            adjacent: vec![Vec::new(); n],
        }
    }

    pub fn max_flow(&mut self, source: usize, target: usize) -> i64 {
        let mut queue = VecDeque::new();
        let mut d = vec![0; self.adjacent.len()];
        let mut flow = 0;
        let mut head = vec![0; self.adjacent.len()];
        while self.bfs(source, target, &mut queue, &mut d) {
            head.fill(0);
            loop {
                let delta = self.dfs(source, target, &mut head, &d, i64::MAX);
                if delta == 0 {
                    break;
                }
                flow += delta;
            }
        }
        flow
    }

    fn bfs(
        &mut self,
        source: usize,
        target: usize,
        queue: &mut VecDeque<usize>,
        d: &mut [usize],
    ) -> bool {
        queue.clear();
        d.fill(usize::MAX);
        d[source] = 0;
        queue.push_back(source);
        while let Some(v) = queue.pop_front() {
            for edge in self.adjacent[v]
                .iter()
                .map(|&index| &self.edges[index])
                .filter(|edge| edge.cap - edge.flow > 0)
            {
                if d[edge.to] == usize::MAX {
                    d[edge.to] = d[v] + 1;
                    queue.push_back(edge.to);
                    if edge.to == target {
                        return true;
                    }
                }
            }
        }
        false
    }

    fn dfs(&mut self, v: usize, target: usize, head: &mut [usize], d: &[usize], c_min: i64) -> i64 {
        if v == target {
            return c_min;
        }
        while let Some(index) = self.adjacent[v].get(head[v]).copied() {
            let edge = &self.edges[index];
            if d[edge.from] + 1 == d[edge.to] {
                let residual = edge.cap - edge.flow;
                if residual > 0 {
                    let delta = self.dfs(edge.to, target, head, d, min(c_min, residual));
                    if delta > 0 {
                        self.edges[index].flow += delta;
                        self.edges[index ^ 1].flow -= delta;
                        return delta;
                    }
                }
            }
            head[v] += 1;
        }
        0
    }

    pub fn add_edge(&mut self, from: usize, to: usize, cap: i64) -> usize {
        self.add_edge_internal(
            Edge {
                from,
                to,
                cap,
                flow: 0,
            },
            Edge {
                from: to,
                to: from,
                cap: 0,
                flow: 0,
            },
        )
    }

    pub fn add_edge_undirected(&mut self, from: usize, to: usize, cap: i64) -> usize {
        self.add_edge_internal(
            Edge {
                from,
                to,
                cap,
                flow: 0,
            },
            Edge {
                from: to,
                to: from,
                cap,
                flow: 0,
            },
        )
    }

    fn add_edge_internal(&mut self, e1: Edge, e2: Edge) -> usize {
        let index = self.edges.len();
        self.adjacent[e1.from].push(index);
        self.adjacent[e2.from].push(index + 1);
        self.edges.push(e1);
        self.edges.push(e2);
        index
    }

    pub fn get_edge_flow(&self, id: usize) -> i64 {
        self.edges[id].flow
    }
}

mod test {
    use crate::graphs::flows::dinic::DinicGraph;

    #[test]
    fn test_simple() {
        let mut g = DinicGraph::new(4);
        g.add_edge(0, 1, 100);
        g.add_edge(0, 2, 100);
        g.add_edge(1, 2, 1);
        g.add_edge(1, 3, 100);
        g.add_edge(2, 3, 100);
        assert_eq!(g.max_flow(0, 3), 200);
    }
}
