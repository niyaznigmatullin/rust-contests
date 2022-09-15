use crate::datastructures::segment_tree::common::SegmentTreeDelta;
use crate::datastructures::segment_tree::common::SegmentTreeNode;
use crate::datastructures::segment_tree::util::create_segment_tree_neutral;
use std::ops::Range;

pub struct DualSegmentTree<T> {
    n: usize,
    t: Vec<T>,
}

impl<T> DualSegmentTree<T>
where
    T: SegmentTreeNode,
{
    pub fn new(n: usize) -> Self {
        let (size, t) = create_segment_tree_neutral(n);
        Self { n: size, t }
    }

    pub fn get(&self, index: usize) -> T {
        let mut i = index + self.n;
        let mut res = T::neutral();
        while i >= 1 {
            res = T::join(&res, &self.t[i]);
            i >>= 1;
        }
        res
    }

    pub fn update<D: SegmentTreeDelta<T>>(&mut self, range: Range<usize>, delta: &D) {
        let mut left = range.start + self.n;
        let mut right = range.end + self.n;
        while left < right {
            if (left & 1) == 1 {
                delta.apply(&mut self.t[left]);
                left += 1;
            }
            if (right & 1) == 1 {
                right -= 1;
                delta.apply(&mut self.t[right]);
            }
            left >>= 1;
            right >>= 1;
        }
    }
}
