//{"name":"D. Минимизируй, прибавляй!","group":"Codeforces - peltorator: Segment Tree Beats","url":"https://codeforces.com/group/1rv4rhCsHp/contest/327313/problem/D","interactive":false,"timeLimit":3000,"tests":[{"input":"3\n1 4 2\n9\n3 1 3\n1 1 3 3\n3 1 3\n1 1 3 1\n3 1 3\n2 1 3 5\n3 1 3\n1 1 3 3\n3 1 3\n","output":"7\n6\n3\n18\n9\n"},{"input":"7\n1 7 2 4 8 4 100\n10\n1 3 6 3\n3 2 7\n1 2 3 5\n2 3 4 -10\n3 1 7\n1 1 7 3\n3 1 4\n3 2 7\n2 1 7 5\n3 1 7\n","output":"118\n97\n-11\n-3\n33\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DMinimiziruiPribavlyai"}}}

use std::cmp::{max, min, Ordering};
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out, out_line};
use algo_lib::datastructures::segment_tree::batch_modification::{SegmentTreeBatch, SegmentTreeBeatsDelta, SegmentTreePushNode};
use algo_lib::datastructures::segment_tree::common::SegmentTreeDelta;

#[derive(Clone, Debug)]
struct TreeNodeDelta {
    upd_up: i64,
    add: i64,
}

impl TreeNodeDelta {
    fn nothing() -> Self {
        Self {
            upd_up: i64::MAX,
            add: 0,
        }
    }

    fn compose(&mut self, other: &Self) {
        if other.upd_up != i64::MAX {
            self.upd_up = min(self.upd_up, other.upd_up - self.add);
        }
        self.add += other.add;
    }
}

#[derive(Clone, Debug)]
struct TreeNodeValue {
    count: usize,
    max: i64,
    second_max: i64,
    count_max: usize,
    sum: i64,
}

#[derive(Clone, Debug)]
enum TreeNode {
    None,
    Single(i64, usize),
    Multiple(TreeNodeValue, TreeNodeDelta),
}

impl TreeNode {
    fn count(&self) -> usize {
        match self {
            Self::None => 0,
            Self::Single(_, count) => *count,
            Self::Multiple(e, _) => e.count,
        }
    }

    fn max(&self) -> i64 {
        match self {
            Self::None => i64::MIN,
            Self::Single(value, _) => *value,
            Self::Multiple(e, delta) => min(e.max, delta.upd_up) + delta.add,
        }
    }

    fn sum(&self) -> i64 {
        match self {
            Self::None => 0,
            Self::Single(value, count) => *value * *count as i64,
            Self::Multiple(e, delta) => {
                e.sum
                    + delta.add * (e.count as i64)
                    + (min(delta.upd_up, e.max) - e.max) * (e.count_max as i64)
            }
        }
    }

    fn second_max(&self) -> i64 {
        match self {
            Self::None | Self::Single(..) => i64::MIN,
            Self::Multiple(e, delta) => e.second_max + delta.add,
        }
    }

    fn count_max(&self) -> usize {
        match self {
            Self::Multiple(e, _) => e.count_max,
            _ => self.count(),
        }
    }

    fn apply_delta(&mut self, delta: &TreeNodeDelta) {
        match self {
            Self::None => {}
            Self::Single(value, _) => {
                *value = min(*value, delta.upd_up) + delta.add;
            }
            Self::Multiple(_, self_delta) => {
                self_delta.compose(delta);
                if self.second_max() == i64::MIN {
                    *self = Self::Single(self.max(), self.count());
                }
            }
        }
    }
}

impl SegmentTreePushNode for TreeNode {
    fn neutral() -> Self {
        Self::None
    }

    fn join(left: &Self, right: &Self) -> Self {
        let new_count = left.count() + right.count();
        if new_count == 0 {
            return Self::None;
        }
        let left_max = left.max();
        let right_max = right.max();
        let new_max = max(left_max, right_max);
        let second_max = {
            match left_max.cmp(&right_max) {
                Ordering::Less => max(left_max, right.second_max()),
                Ordering::Equal => max(left.second_max(), right.second_max()),
                Ordering::Greater => max(right_max, left.second_max()),
            }
        };
        if second_max == i64::MIN {
            return Self::Single(new_max, new_count);
        }
        Self::Multiple(
            TreeNodeValue {
                count: new_count,
                max: new_max,
                second_max,
                count_max: {
                    match left_max.cmp(&right_max) {
                        Ordering::Less => right.count_max(),
                        Ordering::Equal => left.count_max() + right.count_max(),
                        Ordering::Greater => left.count_max(),
                    }
                },
                sum: left.sum() + right.sum(),
            },
            TreeNodeDelta::nothing(),
        )
    }

    fn push(&mut self, left: &mut Self, right: &mut Self) {
        match self {
            Self::None => panic!("unreachable"),
            Self::Single(value, _) => {
                match left {
                    Self::None => {}
                    _ => {
                        *left = TreeNode::Single(*value, left.count());
                    }
                }
                match right {
                    Self::None => {}
                    _ => {
                        *right = TreeNode::Single(*value, right.count());
                    }
                }
            }
            Self::Multiple(_, delta) => {
                left.apply_delta(delta);
                right.apply_delta(delta);
            }
        }
        *self = Self::join(left, right);
    }
}

struct MinAssign {
    x: i64,
}

impl SegmentTreeBeatsDelta<TreeNode> for MinAssign {
    fn apply(&self, node: &mut TreeNode) {
        assert!(self.tag_condition(node));
        node.apply_delta(&TreeNodeDelta {
            upd_up: self.x,
            add: 0,
        });
    }

    fn tag_condition(&self, node: &TreeNode) -> bool {
        node.second_max() <= self.x && self.x < node.max()
    }

    fn break_condition(&self, node: &TreeNode) -> bool {
        node.max() <= self.x
    }
}

struct AddAssign {
    x: i64,
}

impl SegmentTreeDelta<TreeNode> for AddAssign {
    fn apply(&self, node: &mut TreeNode) {
        node.apply_delta(&TreeNodeDelta {
            upd_up: i64::MAX,
            add: self.x,
        });
    }
}

impl From<i64> for TreeNode {
    fn from(value: i64) -> Self {
        Self::Single(value, 1)
    }
}

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read();
    let a = input.read_vec::<i64>(n);
    let mut tree = SegmentTreeBatch::<TreeNode>::from_vec(a);
    let q = input.read();
    for _ in 0..q {
        let (op, left, right): (i32, usize, usize) = input.read();
        let range = (left - 1)..right;
        match op {
            1 => tree.change_beats(range, &MinAssign { x: input.read() }),
            2 => tree.change(range, &AddAssign { x: input.read() }),
            _ => {
                out_line!(tree.compute(range).sum());
            }
        }
    }
}

pub(crate)
fn run(mut input: Input) -> bool {
    solve(&mut input, 1);
    output().flush();
    input.skip_whitespace();
    !input.peek().is_some()
}


//START MAIN
mod tester;

fn main() {
    tester::run_tests();
}
//END MAIN
