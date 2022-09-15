pub trait SegmentTreeNode {
    fn neutral() -> Self;
    fn join(left: &Self, right: &Self) -> Self;
}

pub trait SegmentTreeDelta<T> {
    fn apply(&self, node: &mut T);
}
