//{"name":"b_ice_skates","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"b_ice_skates"}}}

use std::cmp::{max, min};
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out, out_line};
use algo_lib::datastructures::segment_tree::common::SegmentTreeNode;
use algo_lib::datastructures::segment_tree::simple::SegmentTree;

#[derive(Copy, Clone)]
struct Node {
    sum: i64,
    max_prefix_sum: i64,
    max_suffix_sum: i64,
    max_sum: i64,
}

impl SegmentTreeNode for Node {
    fn neutral() -> Self {
        Self {
            sum: 0,
            max_prefix_sum: 0,
            max_suffix_sum: 0,
            max_sum: 0,
        }
    }

    fn join(left: &Self, right: &Self) -> Self {
        Self {
            sum: left.sum + right.sum,
            max_prefix_sum: max(left.max_prefix_sum, left.sum + right.max_prefix_sum),
            max_suffix_sum: max(right.max_suffix_sum, right.sum + left.max_suffix_sum),
            max_sum: max(max(left.max_sum, right.max_sum), left.max_suffix_sum + right.max_prefix_sum),
        }
    }
}

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read();
    let m = input.read();
    let k: i64 = input.read();
    let d: i64 = input.read();
    // let mut prefix = {
    //     let mut for_prefix = vec![Node::neutral(); n];
    //     for i in 0..n {
    //         for_prefix[i.saturating_sub(d)].sum += k;
    //     }
    //     SegmentTree::<Node>::from_vec(for_prefix)
    // };
    let mut suffix = {
        let node = Node {
            sum: -k,
            ..Node::neutral()
        };
        SegmentTree::<Node>::from_vec(vec![node; n])
    };
    for _ in 0..m {
        let size: usize = input.read();
        let count: i64 = input.read();
        // prefix.change(size - 1, |x| x.sum -= count);
        suffix.change(size - 1, |x| x.sum += count);
        let max_sum = suffix.compute(0..n).max_sum;
        let ans = max_sum <= d as i64 * k;
        out_line!(if ans { "TAK" } else { "NIE" });
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
