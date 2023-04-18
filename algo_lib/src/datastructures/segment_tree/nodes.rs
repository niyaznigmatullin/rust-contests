use crate::datastructures::segment_tree::common::SegmentTreeNode;
use std::ops::Add;

#[derive(Clone, Copy)]
pub struct SumNode<V> {
    pub sum: V,
}

impl<V> SumNode<V> {
    pub fn new(sum: V) -> Self {
        Self { sum }
    }
}

impl<V> Default for SumNode<V>
where
    SumNode<V>: SegmentTreeNode,
{
    fn default() -> Self {
        Self::neutral()
    }
}

impl<V> SegmentTreeNode for SumNode<V>
where
    V: Default + Add<Output = V> + Copy + Clone,
{
    fn neutral() -> Self {
        Self { sum: V::default() }
    }

    fn join(left: &Self, right: &Self) -> Self {
        Self {
            sum: left.sum + right.sum,
        }
    }
}
