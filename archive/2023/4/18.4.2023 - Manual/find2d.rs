//{"name":"find2d","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"find2d"}}}

use algo_lib::datastructures::segment_tree::nodes::SumNode;
use algo_lib::datastructures::segment_tree::persistent::simple::PersistentSegmentTree;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out, out_line};
use std::mem::swap;

struct Generator {
    cur: u32,
    a: u32,
    b: u32,
}

impl Generator {
    fn new(a: u32, b: u32) -> Self {
        Self { cur: 0, a, b }
    }

    fn next_rand24(&mut self) -> u32 {
        self.next_state();
        self.cur >> 8
    }

    fn next_rand17(&mut self) -> u32 {
        self.next_state();
        self.cur >> 15
    }

    fn next_state(&mut self) {
        self.cur = self.cur.overflowing_mul(self.a).0.overflowing_add(self.b).0;
    }

    fn update(&mut self, ans: u32) {
        self.b = self.b.overflowing_add(ans).0;
    }
}

struct TwoDFinder {
    values: Vec<u32>,
    versions: Vec<PersistentSegmentTree<SumNode<usize>>>,
}

impl TwoDFinder {
    fn new(v: Vec<u32>) -> Self {
        let mut v = v.into_iter().enumerate().collect::<Vec<_>>();
        v.sort_by_key(|(_, x)| *x);
        let mut versions = vec![PersistentSegmentTree::<SumNode<usize>>::build_tree(v.len())];
        for &(i, _) in v.iter() {
            let last_version = versions.last().unwrap();
            let new_version = last_version.change(i, |x| x.sum += 1);
            versions.push(new_version);
        }
        Self {
            values: v.into_iter().map(|x| x.1).collect(),
            versions,
        }
    }

    fn calculate(&self, l: usize, r: usize, x: u32, y: u32) -> usize {
        let left_version = {
            let pos = self.values.partition_point(|&w| w < x);
            &self.versions[pos]
        };
        let right_version = {
            let pos = self.values.partition_point(|&w| w <= y);
            &self.versions[pos]
        };
        let left_count = left_version.compute(l..(r + 1)).sum;
        let right_count = right_version.compute(l..(r + 1)).sum;
        right_count - left_count
    }
}

fn solve(input: &mut Input, _test_case: usize) {
    let q = input.read();
    let a = input.read();
    let b = input.read();
    let mut g = Generator::new(a, b);
    let n = 1 << 17;
    let mut f = vec![0; n];
    for i in 0..n {
        f[i] = g.next_rand24();
    }
    let finder = TwoDFinder::new(f.clone());
    let mut sum = 0u32;
    for _ in 0..q {
        let mut l = g.next_rand17();
        let mut r = g.next_rand17();
        if l > r {
            swap(&mut l, &mut r);
        }
        let mut x = g.next_rand24();
        let mut y = g.next_rand24();
        if x > y {
            swap(&mut x, &mut y);
        }
        let ans = finder.calculate(l as usize, r as usize, x, y) as u32;
        g.update(ans);
        sum = sum.overflowing_add(ans).0;
    }
    out_line!(sum);
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
