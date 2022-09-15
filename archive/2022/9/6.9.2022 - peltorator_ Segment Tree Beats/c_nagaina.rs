//{"name":"C. Нагайна","group":"Codeforces - peltorator: Segment Tree Beats","url":"https://codeforces.com/group/1rv4rhCsHp/contest/327313/problem/C","interactive":false,"timeLimit":4000,"tests":[{"input":"3\n1 1 10 10\n1 2 4 -7\n2 1 10\n","output":"34\n"},{"input":"7\n1 2 3 5\n1 1 10 10\n1 4 5 -5\n2 4 8\n1 1 10 -10\n2 4 8\n2 1 10\n","output":"15\n75\n170\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CNagaina"}}}

use algo_lib::datastructures::segment_tree::batch_modification::{
    SegmentTreeBatch, SegmentTreeBeatsDelta, SegmentTreePushNode,
};
use algo_lib::datastructures::segment_tree::common::{SegmentTreeDelta, SegmentTreeNode};
use algo_lib::datastructures::segment_tree::dual::DualSegmentTree;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out, out_line};
use std::cmp::{max, min};
use std::collections::BTreeSet;
use std::ops::Range;

struct MinValue {
    x: i64,
}

impl SegmentTreeNode for MinValue {
    fn neutral() -> Self {
        Self { x: i64::MAX }
    }

    fn join(left: &Self, right: &Self) -> Self {
        Self {
            x: min(left.x, right.x),
        }
    }
}

impl SegmentTreeDelta<MinValue> for MinValue {
    fn apply(&self, node: &mut MinValue) {
        node.x = min(node.x, self.x);
    }
}

#[derive(Clone, Debug)]
struct MinSum {
    max: i64,
    second_max: i64,
    count_max: usize,
    sum: i64,
    upd: i64,
}

impl MinSum {
    fn from_value(value: i64) -> Self {
        Self {
            max: value,
            second_max: i64::MIN,
            count_max: 1,
            sum: value,
            upd: i64::MAX,
        }
    }

    fn max(&self) -> i64 {
        min(self.upd, self.max)
    }

    fn sum(&self) -> i64 {
        self.sum + (self.count_max as i64) * (self.max() - self.max)
    }
}

impl SegmentTreePushNode for MinSum {
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

impl Default for MinSum {
    fn default() -> Self {
        MinSum::from_value(0)
    }
}

struct MinWith {
    x: i64,
}

impl SegmentTreeBeatsDelta<MinSum> for MinWith {
    fn apply(&self, node: &mut MinSum) {
        node.upd = min(node.upd, self.x);
    }

    fn tag_condition(&self, node: &MinSum) -> bool {
        node.second_max <= self.x && self.x < node.max()
    }

    fn break_condition(&self, node: &MinSum) -> bool {
        node.max() <= self.x
    }
}

struct Half {
    simple: DualSegmentTree<MinValue>,
    for_sum: SegmentTreeBatch<MinSum>,
    closed: BTreeSet<usize>,
}

impl Half {
    fn new(n: usize) -> Self {
        Self {
            simple: DualSegmentTree::new(n),
            for_sum: SegmentTreeBatch::new(n),
            closed: (0..n).collect(),
        }
    }

    fn relax(&mut self, range: Range<usize>, value: i64) {
        self.simple.update(range.clone(), &MinValue { x: value });
        self.for_sum.change_beats(range, &MinWith { x: value });
    }

    fn open(&mut self, range: Range<usize>, other: &mut Half) {
        let to_open = self.closed.range(range).copied().collect::<Vec<_>>();
        for &index in &to_open {
            if !other.closed.contains(&index) {
                self.open_index(index);
                other.open_index(index);
            }
        }
        for value in to_open {
            self.closed.remove(&value);
        }
    }

    fn open_index(&mut self, index: usize) {
        let value = self.simple.get(index).x;
        self.for_sum.set(index, MinSum::from_value(value));
    }

    fn get_sum(&mut self, range: Range<usize>) -> i64 {
        self.for_sum.compute(range).sum()
    }
}

fn solve(input: &mut Input, _test_case: usize) {
    let q = input.read();
    let n = 100000;
    let mut positive = Half::new(n);
    let mut negative = Half::new(n);
    for _ in 0..q {
        let (op, left, right): (i32, usize, usize) = input.read();
        let range = left..right;
        if op == 1 {
            let k = input.read();
            if k > 0 {
                positive.relax(range.clone(), k);
                negative.open(range, &mut positive);
            } else {
                negative.relax(range.clone(), -k);
                positive.open(range, &mut negative);
            }
        } else {
            out_line!(positive.get_sum(range.clone()) + negative.get_sum(range));
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
