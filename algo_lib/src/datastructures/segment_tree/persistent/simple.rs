use crate::datastructures::segment_tree::common::SegmentTreeNode;
use std::ops::Range;
use std::rc::Rc;

pub struct PSegTreeNode<T> {
    pub value: T,
    pub left: Option<Rc<Self>>,
    pub right: Option<Rc<Self>>,
}

impl<T> PSegTreeNode<T>
where
    T: SegmentTreeNode,
{
    fn new_leaf(value: T) -> Self {
        Self {
            value,
            left: None,
            right: None,
        }
    }

    fn new_from_children(left: Rc<Self>, right: Rc<Self>) -> Self {
        let value = T::join(&left.value, &right.value);
        Self {
            value,
            left: Some(left),
            right: Some(right),
        }
    }

    pub fn get_left(&self) -> &Rc<Self> {
        self.left.as_ref().unwrap()
    }

    pub fn get_right(&self) -> &Rc<Self> {
        self.right.as_ref().unwrap()
    }
}

pub struct PersistentSegmentTree<T> {
    pub root: Rc<PSegTreeNode<T>>,
    n: usize,
}

impl<T> PersistentSegmentTree<T>
where
    T: SegmentTreeNode + Clone,
{
    pub fn build_tree(n: usize) -> Self
    where
        T: Default,
    {
        let root = Rc::new(Self::build_tree_internal(0..n));
        Self { root, n }
    }

    fn build_tree_internal(range: Range<usize>) -> PSegTreeNode<T>
    where
        T: Default,
    {
        if range.start + 1 == range.end {
            return PSegTreeNode::new_leaf(T::default());
        }
        let mid = (range.start + range.end) >> 1;
        let left_child = Self::build_tree_internal(range.start..mid);
        let right_child = Self::build_tree_internal(mid..range.end);
        PSegTreeNode::new_from_children(Rc::new(left_child), Rc::new(right_child))
    }

    pub fn set(&self, index: usize, value: T) -> Self {
        self.change_internal(index, move |_| value)
    }

    pub fn change(&self, index: usize, delta: impl FnOnce(&mut T) -> ()) -> Self {
        self.change_internal(index, move |x| {
            let mut y = x.clone();
            delta(&mut y);
            y
        })
    }

    fn change_internal(&self, index: usize, delta: impl FnOnce(&T) -> T) -> Self {
        let root = Self::change_internal_impl(&self.root, 0..self.n, index, delta);
        Self {
            root: Rc::new(root),
            n: self.n,
        }
    }

    fn change_internal_impl(
        v: &PSegTreeNode<T>,
        range: Range<usize>,
        index: usize,
        f: impl FnOnce(&T) -> T,
    ) -> PSegTreeNode<T> {
        if range.start + 1 == range.end {
            return PSegTreeNode::new_leaf(f(&v.value));
        }
        let mid = (range.start + range.end) >> 1;
        let left = v.get_left();
        let right = v.get_right();
        if index < mid {
            let new_left = Self::change_internal_impl(left, range.start..mid, index, f);
            PSegTreeNode::new_from_children(Rc::new(new_left), right.clone())
        } else {
            let new_right = Self::change_internal_impl(right, mid..range.end, index, f);
            PSegTreeNode::new_from_children(left.clone(), Rc::new(new_right))
        }
    }

    pub fn compute(&self, range: Range<usize>) -> T {
        Self::compute_internal(&self.root, 0..self.n, range)
    }

    fn compute_internal(v: &PSegTreeNode<T>, v_range: Range<usize>, range: Range<usize>) -> T {
        if range.start <= v_range.start && v_range.end <= range.end {
            return v.value.clone();
        }
        if range.end <= v_range.start || v_range.end <= range.start {
            return T::neutral();
        }
        let mid = (v_range.start + v_range.end) >> 1;
        T::join(
            &Self::compute_internal(v.get_left(), v_range.start..mid, range.clone()),
            &Self::compute_internal(v.get_right(), mid..v_range.end, range),
        )
    }
}
