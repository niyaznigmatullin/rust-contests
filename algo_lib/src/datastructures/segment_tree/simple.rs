use crate::datastructures::segment_tree::util::{create_segment_tree_default, create_segment_tree_vec};
use std::ops::Range;
use crate::datastructures::segment_tree::common::SegmentTreeNode;

pub struct SegmentTree<T> {
    n: usize,
    t: Vec<T>,
}

impl<T> SegmentTree<T>
where
    T: SegmentTreeNode,
{
    pub fn new(n: usize) -> Self
    where
        T: Default,
    {
        let (size, t) = create_segment_tree_default(n);
        Self { n: size, t }
    }

    pub fn from_vec<V: Into<T>>(a: Vec<V>) -> Self {
        let (size, t) = create_segment_tree_vec(a);
        Self { n: size, t }
    }

    pub fn set(&mut self, index: usize, value: T) {
        let mut i = index + self.n;
        let t = &mut self.t;
        t[i] = value;
        while i > 1 {
            i >>= 1;
            t[i] = T::join(&t[i + i], &t[i + i + 1]);
        }
    }

    pub fn compute(&self, range: Range<usize>) -> T {
        let mut l_result = T::neutral();
        let mut r_result = T::neutral();
        let mut left = range.start + self.n;
        let mut right = range.end + self.n;
        while left < right {
            if (left & 1) == 1 {
                l_result = T::join(&l_result, &self.t[left]);
                left += 1
            }
            if (right & 1) == 1 {
                right -= 1;
                r_result = T::join(&self.t[right], &r_result);
            }
            left >>= 1;
            right >>= 1;
        }
        T::join(&l_result, &r_result)
    }
}
