//{"name":"D1: Work-Life Balance - Chapter 1","group":"Meta Coding Competitions - Meta Hacker Cup 2022 Round 2","url":"https://www.facebook.com/codingcompetitions/hacker-cup/2022/round-2/problems/D1","interactive":false,"timeLimit":360000,"tests":[{"input":"3\n2 2\n1 2\n2 1 1\n2 3 1\n4 2\n1 1 3 3\n4 1 3\n1 3 2\n6 4\n1 2 3 3 3 3\n6 2 3\n4 1 2\n3 1 2\n1 3 5\n","output":"Case #1: -1\nCase #2: 1\nCase #3: 3\n"}],"testType":"multiNumber","input":{"type":"regex","fileName":null,"pattern":"worklife_balance_chapter__.*input[.]txt"},"output":{"type":"file","fileName":"worklife_balance_chapter__output.txt","pattern":null},"languages":{"java":{"taskClass":"D1WorkLifeBalanceChapter1"}}}

use std::cmp::min;
use algo_lib::datastructures::segment_tree::common::SegmentTreeNode;
use algo_lib::datastructures::segment_tree::simple::SegmentTree;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out, out_line};

#[derive(Copy, Clone, Default, Debug)]
struct CountNode {
    c1: usize,
    c2: usize,
    c3: usize,
}

impl SegmentTreeNode for CountNode {
    fn neutral() -> Self {
        Self {
            c1: 0,
            c2: 0,
            c3: 0,
        }
    }

    fn join(left: &Self, right: &Self) -> Self {
        Self {
            c1: left.c1 + right.c1,
            c2: left.c2 + right.c2,
            c3: left.c3 + right.c3,
        }
    }
}

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read();
    let m = input.read();
    let mut a = input.read_vec::<usize>(n);
    let nodes = vec![
        CountNode {
            c1: 1,
            c2: 0,
            c3: 0,
        },
        CountNode {
            c1: 0,
            c2: 1,
            c3: 0,
        },
        CountNode {
            c1: 0,
            c2: 0,
            c3: 1,
        },
    ];
    let mut counts = SegmentTree::from_vec(a.iter().map(|x| nodes[*x - 1]).collect());
    let mut sum = a.iter().sum::<usize>();
    let mut ans = 0i64;
    for _ in 0..m {
        let (x, y, z): (usize, usize, usize) = input.read();
        let x = x - 1;
        sum -= a[x];
        a[x] = y;
        sum += a[x];
        // eprintln!("assign a[{}] = {}", x, y);
        counts.set(x, nodes[a[x] - 1]);
        let count_left = counts.compute(0..z);
        let count_right = counts.compute(z..n);
        // eprintln!("     {:?} {:?} sum = {}", count_left, count_right, sum);
        let result = count_all(sum, count_left, count_right);
        // eprintln!(" result = {}", result);
        ans += result;
    }
    out_line!(format!("Case #{}: {}", _test_case, ans));
}

fn count_all(sum: usize, count_left: CountNode, count_right: CountNode) -> i64 {
    if sum % 2 != 0 {
        return -1;
    }
    let half = sum / 2;
    let sum_left = count_left.c1 * 1 + count_left.c2 * 2 + count_left.c3 * 3;
    if sum_left < half {
        compute(count_left, count_right, half - sum_left)
    } else {
        compute(count_right, count_left, sum_left - half)
    }
}

fn compute(mut less: CountNode, mut more: CountNode, mut delta: usize) -> i64 {
    if delta == 0 {
        return 0;
    }
    // eprintln!("delta = {}, less = {:?}, more = {:?}", delta, less, more);
    let by_two = min(delta / 2, min(less.c1, more.c3));
    // eprintln!("by_two = {}", by_two);
    let mut ans = 0;
    ans += by_two;
    delta -= by_two * 2;
    less.c1 -= by_two;
    less.c3 += by_two;
    more.c3 -= by_two;
    more.c1 += by_two;
    let e12 = min(delta, min(less.c1, more.c2));
    less.c1 -= e12;
    more.c2 -= e12;
    delta -= e12;
    ans += e12;
    less.c2 += e12;
    more.c2 += e12;
    let e23 = min(delta, min(less.c2, more.c3));
    delta -= e23;
    ans += e23;
    // eprintln!("delta left = {}", delta);
    if delta > 0 {
        return -1;
    }
    ans as i64
}

pub(crate) fn run(mut input: Input) -> bool {
    let t = input.read();
    for i in 0usize..t {
        solve(&mut input, i + 1);
    }
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
