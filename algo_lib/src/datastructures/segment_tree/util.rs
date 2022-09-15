use crate::datastructures::segment_tree::common::SegmentTreeNode;

fn build<T: SegmentTreeNode>(size: usize, mut t: Vec<T>) -> Vec<T> {
    for i in (1..size).rev() {
        t[i] = T::join(&t[i + i], &t[i + i + 1]);
    }
    t
}

pub fn create_segment_tree_default<T: SegmentTreeNode + Default>(n: usize) -> (usize, Vec<T>) {
    let (size, mut t) = create_segment_tree_neutral(n);
    t.split_at_mut(size)
        .1
        .iter_mut()
        .for_each(|x| *x = T::default());
    (size, build(size, t))
}

pub fn create_segment_tree_neutral<T: SegmentTreeNode>(n: usize) -> (usize, Vec<T>) {
    let size = calc_size(n);
    (size, (0..(size * 2)).map(|_| T::neutral()).collect())
}

pub fn create_segment_tree_vec<T: SegmentTreeNode, V: Into<T>>(a: Vec<V>) -> (usize, Vec<T>) {
    let (size, mut t) = create_segment_tree_neutral(a.len());
    t.split_at_mut(size)
        .1
        .iter_mut()
        .zip(a.into_iter())
        .for_each(|(node, v)| *node = v.into());
    (size, build(size, t))
}

fn calc_size(n: usize) -> usize {
    let mut size = 1;
    while size < n {
        size *= 2;
    }
    size
}
