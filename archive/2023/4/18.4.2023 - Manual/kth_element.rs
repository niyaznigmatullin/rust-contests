//{"name":"kth_element","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"kth_element"}}}

use algo_lib::datastructures::segment_tree::nodes::SumNode;
use algo_lib::datastructures::segment_tree::persistent::simple::PersistentSegmentTree;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out, out_line};
use std::cmp::{max, min};
use std::ops::Range;

struct KthElementFinder {
    versions: Vec<PersistentSegmentTree<SumNode<usize>>>,
    values: usize,
}

impl KthElementFinder {
    fn new(a: Vec<usize>) -> Self {
        let values = a.iter().max().unwrap() + 1;
        let empty_tree = PersistentSegmentTree::<SumNode<usize>>::build_tree(values);
        let mut versions = vec![empty_tree];
        for i in 0..a.len() {
            let last_version = versions.last().unwrap();
            let new_version = last_version.change(a[i], |x| x.sum += 1);
            versions.push(new_version);
        }
        Self { versions, values }
    }

    fn kth(&self, range: Range<usize>, mut k: usize) -> usize {
        let mut version1 = &self.versions[range.start].root;
        let mut version2 = &self.versions[range.end].root;
        let mut left = 0;
        let mut right = self.values;
        while left + 1 < right {
            let mid = (left + right) >> 1;
            let left1 = version1.get_left();
            let left2 = version2.get_left();
            let count_left = left2.value.sum - left1.value.sum;
            if count_left >= k {
                version1 = left1;
                version2 = left2;
                right = mid;
            } else {
                k -= count_left;
                version1 = version1.get_right();
                version2 = version2.get_right();
                left = mid;
            }
        }
        left
    }
}

fn compress(a: Vec<i32>) -> (Vec<usize>, Vec<i32>) {
    let mut b = a.clone();
    b.sort();
    b.dedup();
    let a = a
        .into_iter()
        .map(|x| b.partition_point(|&y| y < x))
        .collect();
    (a, b)
}

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read();
    let a1 = input.read();
    let l: i64 = input.read();
    let m: i64 = input.read();
    let mut a = vec![0; n];
    a[0] = a1;
    for i in 1..n {
        a[i] = (((a[i - 1] as i64) * l + m) % 1000000000) as i32;
    }
    // dbg!(&a);
    let (a, b) = compress(a);
    let finder = KthElementFinder::new(a);
    let blocks = input.read();
    let mut ans = 0;
    let n_i64 = n as i64;
    for _ in 0..blocks {
        let g = input.read();
        let mut x1 = input.read();
        let lx: i64 = input.read();
        let mx: i64 = input.read();
        let mut y1 = input.read();
        let ly: i64 = input.read();
        let my: i64 = input.read();
        let mut k1 = input.read();
        let lk: i64 = input.read();
        let mk: i64 = input.read();
        let mut ig = min(x1, y1);
        let mut jg = max(x1, y1);
        for _ in 0..g {
            let current_ans = b[finder.kth((ig - 1)..jg, k1)];
            // dbg!(current_ans);
            ans += current_ans as i64;
            x1 = (((ig - 1) as i64 * lx + mx) % n_i64 + 1) as usize;
            y1 = (((jg - 1) as i64 * ly + my) % n_i64 + 1) as usize;
            ig = min(x1, y1);
            jg = max(x1, y1);
            k1 = (((k1 - 1) as i64 * lk + mk) % ((jg - ig + 1) as i64) + 1) as usize;
        }
    }
    out_line!(ans);
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
