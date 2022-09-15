//{"name":"B. Минимизируй!","group":"Codeforces - peltorator: Segment Tree Beats","url":"https://codeforces.com/group/1rv4rhCsHp/contest/327313/problem/B","interactive":false,"timeLimit":2000,"tests":[{"input":"3\n1 4 2\n5\n2 1 3\n1 1 3 3\n2 1 3\n1 1 3 1\n2 1 3\n","output":"7\n6\n3\n"},{"input":"7\n1 7 2 4 8 4 100\n7\n1 3 6 3\n2 2 7\n1 2 3 5\n2 1 7\n1 1 7 3\n2 1 4\n2 2 7\n","output":"118\n117\n9\n17\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BMinimizirui"}}}

use algo_lib::datastructures::segment_tree::batch_modification::{
    SegmentTreeBatch, SegmentTreeBeatsDelta, SegmentTreePushNode,
};
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out, out_line};
use std::cmp::{max, min};

#[derive(Clone)]
struct TreeNode {
    max: i64,
    second_max: i64,
    count_max: usize,
    sum: i64,
    upd: i64,
}

impl TreeNode {
    fn max(&self) -> i64 {
        min(self.upd, self.max)
    }

    fn sum(&self) -> i64 {
        self.sum + (self.count_max as i64) * (self.max() - self.max)
    }
}

impl SegmentTreePushNode for TreeNode {
    fn neutral() -> Self {
        Self {
            max: i64::MIN,
            second_max: i64::MIN,
            count_max: 0,
            sum: 0,
            upd: i64::MAX,
        }
    }

    fn join(left: &Self, right: &Self) -> Self {
        Self {
            max: max(left.max(), right.max()),
            second_max: {
                if left.max() > right.max() {
                    max(right.max(), left.second_max)
                } else if right.max() > left.max() {
                    max(left.max(), right.second_max)
                } else {
                    max(left.second_max, right.second_max)
                }
            },
            count_max: {
                if left.max() > right.max() {
                    left.count_max
                } else if right.max() > left.max() {
                    right.count_max
                } else {
                    left.count_max + right.count_max
                }
            },
            sum: left.sum() + right.sum(),
            upd: i64::MAX,
        }
    }

    fn push(&mut self, left: &mut Self, right: &mut Self) {
        if self.upd == i64::MAX {
            return;
        }
        left.upd = min(left.upd, self.upd);
        right.upd = min(right.upd, self.upd);
        let new_self = Self {
            max: self.max(),
            second_max: self.second_max,
            count_max: self.count_max,
            sum: self.sum(),
            upd: i64::MAX,
        };
        *self = new_self;
    }
}

impl From<i64> for TreeNode {
    fn from(value: i64) -> Self {
        Self {
            max: value,
            second_max: i64::MIN,
            count_max: 1,
            sum: value,
            upd: i64::MAX,
        }
    }
}

struct MinWith {
    x: i64,
}

impl SegmentTreeBeatsDelta<TreeNode> for MinWith {
    fn apply(&self, node: &mut TreeNode) {
        node.upd = min(node.upd, self.x);
    }

    fn tag_condition(&self, node: &TreeNode) -> bool {
        node.second_max <= self.x && self.x < node.max()
    }

    fn break_condition(&self, node: &TreeNode) -> bool {
        node.max() <= self.x
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
        if op == 1 {
            let x = input.read();
            tree.change_beats(range, &MinWith { x });
        } else {
            out_line!(tree.compute(range).sum());
        }
    }
}

pub(crate) fn run(mut input: Input) -> bool {
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
