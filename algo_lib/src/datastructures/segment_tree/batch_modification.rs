use crate::datastructures::segment_tree::common::SegmentTreeDelta;
use crate::datastructures::segment_tree::common::SegmentTreeNode;
use crate::datastructures::segment_tree::util::{
    create_segment_tree_default, create_segment_tree_vec,
};
use std::mem;
use std::ops::Range;

pub struct SegmentTreeBatch<T> {
    n: usize,
    t: Vec<T>,
}

impl<T> SegmentTreeBatch<T>
where
    T: SegmentTreePushNode,
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

    fn set_internal(&mut self, v: usize, range: Range<usize>, index: usize, value: T) {
        if range.len() == 1 {
            self.t[v] = value;
            return;
        }
        self.push(v);
        let mid = (range.start + range.end) >> 1;
        if index < mid {
            self.set_internal(v + v, range.start..mid, index, value);
        } else {
            self.set_internal(v + v + 1, mid..range.end, index, value);
        }
        self.t[v] = T::join(&self.t[v + v], &self.t[v + v + 1]);
    }

    pub fn set(&mut self, index: usize, value: T) {
        self.set_internal(1, 0..self.n, index, value);
    }

    fn compute_internal(&mut self, v: usize, range: Range<usize>, q_range: &Range<usize>) -> T {
        if q_range.end <= range.start || range.end <= q_range.start {
            return T::neutral();
        }
        if q_range.start <= range.start && range.end <= q_range.end {
            return self.t[v].clone();
        }
        self.push(v);
        let mid = (range.start + range.end) >> 1;
        let res = T::join(
            &self.compute_internal(v + v, range.start..mid, q_range),
            &self.compute_internal(v + v + 1, mid..range.end, q_range),
        );
        self.t[v] = T::join(&self.t[v + v], &self.t[v + v + 1]);
        res
    }

    fn push(&mut self, v: usize) {
        let mut placeholder = T::neutral();
        mem::swap(&mut self.t[v], &mut placeholder);
        placeholder.push(&mut self.t[v + v]);
        placeholder.push(&mut self.t[v + v + 1]);
    }

    pub fn compute(&mut self, range: Range<usize>) -> T {
        self.compute_internal(1, 0..self.n, &range)
    }

    fn change_internal<D: SegmentTreeBeatsDelta<T>>(
        &mut self,
        v: usize,
        range: Range<usize>,
        q_range: &Range<usize>,
        delta: &D,
    ) {
        if q_range.end <= range.start
            || range.end <= q_range.start
            || delta.break_condition(&self.t[v])
        {
            return;
        }
        if q_range.start <= range.start
            && range.end <= q_range.end
            && (v >= self.n || delta.tag_condition(&self.t[v]))
        {
            delta.apply(&mut self.t[v]);
            return;
        }
        self.push(v);
        let mid = (range.start + range.end) >> 1;
        self.change_internal(v + v, range.start..mid, q_range, delta);
        self.change_internal(v + v + 1, mid..range.end, q_range, delta);
        self.t[v] = T::join(&self.t[v + v], &self.t[v + v + 1]);
    }

    pub fn change<D: SegmentTreeBeatsDelta<T>>(&mut self, range: Range<usize>, delta: &D) {
        self.change_internal(1, 0..self.n, &range, delta)
    }
}

pub trait SegmentTreeBeatsDelta<T> {
    fn apply(&self, node: &mut T);
    fn tag_condition(&self, node: &T) -> bool;
    fn break_condition(&self, node: &T) -> bool;
}

impl<D, T> SegmentTreeBeatsDelta<T> for D
where
    D: SegmentTreeDelta<T>,
{
    fn apply(&self, node: &mut T) {
        self.apply(node);
    }

    fn tag_condition(&self, _node: &T) -> bool {
        true
    }

    fn break_condition(&self, _node: &T) -> bool {
        false
    }
}

pub trait SegmentTreePushNode: Clone {
    fn neutral() -> Self;
    fn join(left: &Self, right: &Self) -> Self;
    fn push(&self, child: &mut Self);
}

impl<T> SegmentTreeNode for T
where
    T: SegmentTreePushNode,
{
    fn neutral() -> Self {
        T::neutral()
    }

    fn join(left: &Self, right: &Self) -> Self {
        T::join(left, right)
    }
}
