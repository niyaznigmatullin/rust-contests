//{"name":"D2: Work-Life Balance - Chapter 2","group":"Meta Coding Competitions - Meta Hacker Cup 2022 Round 2","url":"https://www.facebook.com/codingcompetitions/hacker-cup/2022/round-2/problems/D2","interactive":false,"timeLimit":360000,"tests":[{"input":"3\n2 3\n1 2\n2 1 1\n1 2 1\n1 2 1\n4 3\n1 1 1 2\n1 2 2\n2 2 2\n4 1 2\n8 5\n1 1 1 1 2 2 2 2\n5 2 4\n7 2 3\n6 2 5\n1 2 4\n3 2 4\n","output":"Case #1: -2\nCase #2: 0\nCase #3: 16\n"}],"testType":"multiNumber","input":{"type":"regex","fileName":null,"pattern":"worklife_balance_chapter__.*input[.]txt"},"output":{"type":"file","fileName":"worklife_balance_chapter__output.txt","pattern":null},"languages":{"java":{"taskClass":"D2WorkLifeBalanceChapter2"}}}

use algo_lib::datastructures::fenwick::kth::FenwickKth;
use algo_lib::datastructures::segment_tree::common::SegmentTreeNode;
use algo_lib::datastructures::segment_tree::simple::SegmentTree;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out, out_line};
use std::cmp::{max, min};

#[derive(Copy, Clone)]
struct InversionNode {
    count1: usize,
    count2: usize,
    count12: i64,
    count21: i64,
}

impl SegmentTreeNode for InversionNode {
    fn neutral() -> Self {
        Self {
            count1: 0,
            count2: 0,
            count12: 0,
            count21: 0,
        }
    }

    fn join(left: &Self, right: &Self) -> Self {
        Self {
            count1: left.count1 + right.count1,
            count2: left.count2 + right.count2,
            count12: left.count12 + right.count12 + left.count1 as i64 * right.count2 as i64,
            count21: left.count21 + right.count21 + left.count2 as i64 * right.count1 as i64,
        }
    }
}

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read();
    let m = input.read();
    let mut a = input.read_vec::<usize>(n);
    let mut finder = vec![FenwickKth::new(n), FenwickKth::new(n)];
    let nodes = vec![
        InversionNode {
            count1: 1,
            count2: 0,
            count12: 0,
            count21: 0,
        },
        InversionNode {
            count1: 0,
            count2: 1,
            count12: 0,
            count21: 0,
        },
    ];
    let mut inversions = SegmentTree::from_vec(a.iter().map(|x| nodes[*x - 1]).collect());
    let mut sum = 0;
    for i in 0..n {
        finder[a[i] - 1].add(i, 1);
        inversions.set(i, nodes[a[i] - 1]);
        sum += a[i];
    }
    let mut ans = 0;
    for _ in 0..m {
        let (x, y, z): (usize, usize, usize) = input.read();
        let x = x - 1;
        sum -= a[x];
        finder[a[x] - 1].remove(x, 1);
        a[x] = y;
        inversions.set(x, nodes[a[x] - 1]);
        finder[a[x] - 1].add(x, 1);
        sum += a[x];
        let result = calculate(n, &finder, &inversions, sum, z);
        ans += result;
    }
    out_line!(format!("Case #{}: {}", _test_case, ans));
}

fn calculate(
    n: usize,
    finder: &Vec<FenwickKth>,
    inversions: &SegmentTree<InversionNode>,
    sum: usize,
    z: usize,
) -> i64 {
    if sum % 2 != 0 {
        return -1;
    }
    let half = sum / 2;
    if half < max(z, n - z) || half > 2 * min(z, n - z) {
        return -1;
    }
    let twos = half - z;
    let ones = 2 * z - half;
    if ones == 0 {
        return match finder[1].get_kth_pos(twos) {
            Some(pos2) => inversions.compute(0..(pos2 + 1)).count12,
            _ => -1,
        };
    }
    if twos == 0 {
        return match finder[0].get_kth_pos(ones) {
            Some(pos1) => inversions.compute(0..(pos1 + 1)).count21,
            _ => -1,
        };
    }
    let pos1 = finder[0].get_kth_pos(ones);
    let pos2 = finder[1].get_kth_pos(twos);
    if pos1.is_none() || pos2.is_none() {
        return -1;
    }
    let pos1 = pos1.unwrap();
    let pos2 = pos2.unwrap();
    if pos1 < z && pos2 < z {
        return 0;
    }
    if pos1 < pos2 {
        inversions.compute((pos1 + 1)..(pos2 + 1)).count12
    } else {
        inversions.compute((pos2 + 1)..(pos1 + 1)).count21
    }
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
