use std::ops::Index;

// pub struct AdjLists {
//     n: usize,
//     edges: Vec<Vec<usize>>,
// }
//
// impl Index<usize> for AdjLists {
//     type Output = impl Iterator<Item = &usize>;
//
//     fn index(&self, index: usize) -> Self::Output {
//         self.edges[index].iter()
//     }
// }
//
// impl AdjLists {
//     pub fn new_from_edges(n: usize, edges_list: &[(usize, usize)]) -> Self {
//         let mut edges = vec![Vec::new(); n];
//         for &(v, u) in edges_list {
//             edges[v].push(u);
//             edges[u].push(v);
//         }
//         let mut result = Self { n, edges };
//         result
//     }
// }
