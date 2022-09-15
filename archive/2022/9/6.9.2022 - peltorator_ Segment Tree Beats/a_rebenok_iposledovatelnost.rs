//{"name":"A. Ребенок и последовательность","group":"Codeforces - peltorator: Segment Tree Beats","url":"https://codeforces.com/group/1rv4rhCsHp/contest/327313/problem/A","interactive":false,"timeLimit":4000,"tests":[{"input":"5 5\n1 2 3 4 5\n2 3 5 4\n3 3 5\n1 2 5\n2 1 3 3\n1 1 3\n","output":"8\n5\n"},{"input":"10 10\n6 9 6 7 6 1 10 10 9 5\n1 3 9\n2 7 10 9\n2 5 10 8\n1 4 7\n3 3 7\n2 7 9 9\n1 2 4\n1 6 6\n1 5 9\n3 1 10\n","output":"49\n15\n23\n1\n9\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"ARebenokIPosledovatelnost"}}}

use algo_lib::datastructures::segment_tree::batch_modification::{
    SegmentTreeBatch, SegmentTreeBeatsDelta, SegmentTreePushNode,
};
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out, out_line};
use std::cmp::{max, min};
//
// struct NaiveTree {
//     a: Vec<i64>,
// }
//
// impl NaiveTree {
//     fn new(n: usize) -> Self {
//         NaiveTree { a: vec![0; n] }
//     }
//
//     fn get_sum(&self, range: Range<usize>) -> i64 {
//         range.map(|x| self.a[x]).sum()
//     }
//
//     fn do_mod(&mut self, range: Range<usize>, value: i64) {
//         for index in range {
//             self.a[index] %= value;
//         }
//     }
//
//     fn set(&mut self, index: usize, value: i64) {
//         self.a[index] = value;
//     }
// }

#[derive(Clone)]
struct ModSum {
    sum: i64,
    max: i64,
    min: i64,
    add: i64,
    count: usize,
}

impl Default for ModSum {
    fn default() -> Self {
        ModSum::from_value(0)
    }
}

struct ModOperation {
    x: i64,
}

impl ModSum {
    fn from_value(value: i64) -> Self {
        Self {
            sum: value,
            max: value,
            min: value,
            count: 1,
            add: 0,
        }
    }

    fn sum(&self) -> i64 {
        self.sum + self.add * (self.count as i64)
    }

    fn max(&self) -> i64 {
        self.max + self.add
    }

    fn min(&self) -> i64 {
        self.min + self.add
    }
}

impl SegmentTreePushNode for ModSum {
    fn neutral() -> Self {
        ModSum {
            sum: 0,
            max: i64::MIN,
            min: i64::MAX,
            count: 0,
            add: 0,
        }
    }

    fn join(left: &Self, right: &Self) -> Self {
        Self {
            sum: left.sum() + right.sum(),
            max: max(left.max(), right.max()),
            min: min(left.min(), right.min()),
            add: 0,
            count: left.count + right.count,
        }
    }

    fn push(&mut self, left: &mut Self, right: &mut Self) {
        if self.add == 0 {
            return;
        }
        left.add += self.add;
        right.add += self.add;
        self.sum = self.sum();
        self.max = self.max();
        self.min = self.min();
        self.add = 0;
    }
}

impl SegmentTreeBeatsDelta<ModSum> for ModOperation {
    fn apply(&self, node: &mut ModSum) {
        assert!(self.tag_condition(node));
        node.add -= node.max() / self.x * self.x
    }

    fn tag_condition(&self, node: &ModSum) -> bool {
        node.min() / self.x == node.max() / self.x
    }

    fn break_condition(&self, node: &ModSum) -> bool {
        node.max() < self.x
    }
}

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read();
    let m = input.read();
    let mut tree = SegmentTreeBatch::<ModSum>::new(n);
    for i in 0..n {
        tree.set(i, ModSum::from_value(input.read()));
    }
    for _ in 0..m {
        let op: i32 = input.read();
        if op == 1 {
            let (mut l, r): (usize, usize) = input.read();
            l -= 1;
            out_line!(tree.compute(l..r).sum());
        } else if op == 2 {
            let (mut l, r): (usize, usize) = input.read();
            l -= 1;
            let x = input.read();
            tree.change_beats(l..r, &ModOperation { x });
        } else {
            let (mut index, value) = input.read();
            index -= 1;
            tree.set(index, ModSum::from_value(value));
        }
    }
}

// fn test() {
//     return;
//     loop {
//         let n = 3;
//         let m = 4;
//         let mut t1 = SegmentTreeBeats::<ModSum>::new(n);
//         let mut t2 = NaiveTree::new(n);
//         let mut rng = rand::thread_rng();
//         let mut qs = String::new();
//         qs.push_str(&format!("{} {}\n", n, m));
//         qs.push_str(&format!("{}\n", &vec!["0".to_string(); n].join(" ")));
//         for _ in 0..m {
//             let op = rng.gen_range(0..3);
//             if op == 0 {
//                 let value = rng.gen_range(0..10);
//                 let index = rng.gen_range(0..n);
//                 t1.set(index, ModSum::from_value(value));
//                 t2.set(index, value);
//                 qs.push_str(&format!("3 {} {}\n", index + 1, value));
//             } else {
//                 let left = rng.gen_range(0..n);
//                 let right = rng.gen_range(left..n);
//                 let range = left..right;
//                 if op == 1 {
//                     let value = rng.gen_range(1..20);
//                     t2.do_mod(range.clone(), value);
//                     t1.change_beats(range.clone(), &ModOperation { x: value });
//                     qs.push_str(&format!("2 {} {} {}\n", range.start + 1, range.end, value));
//                 } else {
//                     qs.push_str(&format!("1 {} {}\n", range.start + 1, range.end));
//                     let ans1 = t1.compute(range.clone()).sum();
//                     let ans2 = t2.get_sum(range.clone());
//                     if ans1 != ans2 {
//                         eprintln!("{}", qs);
//                         eprintln!("{} {}", ans1, ans2);
//                         return;
//                     }
//                 }
//             }
//         }
//     }
// }

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
