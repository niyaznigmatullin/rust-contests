use std::cmp::min;
use std::ops::Range;

pub struct SparseTable<T: Copy + Ord> {
    table: Vec<Vec<T>>,
    h: Vec<usize>,
}

impl<T: Copy + Ord> SparseTable<T> {
    pub fn new(a: Vec<T>) -> Self {
        let n = a.len();
        let mut h = vec![0; n + 1];
        for i in 2..=n {
            h[i] = h[i >> 1] + 1;
        }
        let mut table = vec![vec![a[0]; n]; h[n] + 1];
        table[0] = a;
        for k in 1..table.len() {
            for i in 0..n {
                table[k][i] = table[k - 1][i];
                let j = i + (1 << (k - 1));
                if j < n {
                    table[k][i] = min(table[k][i], table[k - 1][j]);
                }
            }
        }
        Self { table, h }
    }

    pub fn min(&self, range: Range<usize>) -> Option<T> {
        if range.is_empty() {
            return None;
        }
        let k = self.h[range.len()];
        Some(min(self.table[k][range.start], self.table[k][range.end - (1 << k)]))
    }
}
