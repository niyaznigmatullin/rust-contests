use std::cmp::{min, Reverse};
use std::collections::BinaryHeap;

pub fn shortest_path_dijkstra_by<T, F>(vertices: &[T], source: usize, mut f: F) -> Vec<i64>
where
    F: FnMut(&T, &T) -> Option<i64>,
{
    let n = vertices.len();
    let mut d = vec![i64::MAX; n];
    let mut used = vec![false; n];
    d[source] = 0;
    while let Some(found) = (0..n)
        .filter(|v| !used[*v] && d[*v] != i64::MAX)
        .min_by_key(|v| d[*v])
    {
        used[found] = true;
        for to in 0..n {
            if let Some(weight) = f(&vertices[found], &vertices[to]) {
                d[to] = min(d[to], d[found] + weight);
            }
        }
    }
    d
}

pub fn shortest_path_sparse_dijkstra_continue(
    edges: &[Vec<(usize, i64)>],
    mut d: Vec<i64>,
) -> Vec<i64> {
    let n = edges.len();
    let mut q = BinaryHeap::from_iter(
        (0..n)
            .filter(|v| d[*v] != i64::MAX)
            .map(|v| Reverse((d[v], v))),
    );
    while let Some(Reverse((distance, v))) = q.pop() {
        if d[v] != distance {
            continue;
        }
        for &(to, weight) in &edges[v] {
            if d[to] > distance + weight {
                d[to] = distance + weight;
                q.push(Reverse((d[to], to)));
            }
        }
    }
    d
}

pub fn shortest_path_sparse_dijkstra(edges: &[Vec<(usize, i64)>], source: usize) -> Vec<i64> {
    let mut d = vec![i64::MAX; edges.len()];
    d[source] = 0;
    shortest_path_sparse_dijkstra_continue(edges, d)
}
