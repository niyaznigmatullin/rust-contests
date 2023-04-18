//{"name":"E. Минимизируй, прибавляй, НОДируй и их друзья","group":"Codeforces - peltorator: Segment Tree Beats","url":"https://codeforces.com/group/1rv4rhCsHp/contest/327313/problem/E","interactive":false,"timeLimit":4000,"tests":[{"input":"7\n1 2 3 4 5 6 7\n20\n4 2 7 10\n5 1 6\n6 1 6\n7 1 6\n8 1 6\n2 1 6 14\n5 2 7\n6 2 7\n7 2 7\n8 2 7\n1 2 7 12\n5 1 6\n6 1 6\n7 1 6\n8 1 6\n3 2 6 15\n5 1 7\n6 1 7\n7 1 7\n8 1 7\n","output":"71\n1\n16\n1\n90\n14\n17\n1\n74\n12\n14\n2\n101\n12\n15\n1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"EMinimiziruiPribavlyaiNODiruiIIkhDruzya"}}}

extern crate core;

use algo_lib::datastructures::segment_tree::batch_modification::{
    SegmentTreeBatch, SegmentTreeBeatsDelta, SegmentTreePushNode,
};
use algo_lib::datastructures::segment_tree::common::SegmentTreeDelta;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::math::euclid::gcd;
use algo_lib::{out, out_line};
use std::cmp::{max, min, Ordering};

#[derive(Clone, Debug)]
struct TreeNodeDelta {
    upd_up: i64,
    upd_down: i64,
    add: i64,
}

impl TreeNodeDelta {
    fn nothing() -> Self {
        Self {
            upd_up: i64::MAX,
            upd_down: i64::MIN,
            add: 0,
        }
    }

    fn compose(&mut self, other: &Self) {
        if other.upd_up != i64::MAX {
            self.upd_up = min(self.upd_up, other.upd_up - self.add);
        }
        if other.upd_down != i64::MIN {
            self.upd_down = max(self.upd_down, other.upd_down - self.add);
        }
        self.add += other.add;
    }
}

#[derive(Clone, Debug)]
struct TreeNodeValue {
    count: usize,
    diff_gcd: u64,
    max: i64,
    second_max: i64,
    count_max: usize,
    min: i64,
    second_min: i64,
    count_min: usize,
    sum: i64,
}

#[derive(Clone, Debug)]
enum TreeNode {
    None,
    Single(i64, usize),
    Multiple(TreeNodeValue, TreeNodeDelta),
}

impl TreeNode {
    fn diff_gcd(&self) -> u64 {
        match self {
            TreeNode::None | TreeNode::Single(..) => 0,
            TreeNode::Multiple(e, _) => e.diff_gcd,
        }
    }

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

    fn min(&self) -> i64 {
        match self {
            Self::None => i64::MAX,
            Self::Single(value, _) => *value,
            Self::Multiple(e, delta) => max(e.min, delta.upd_down) + delta.add,
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
                    + (max(delta.upd_down, e.min) - e.min) * (e.count_min as i64)
            }
        }
    }

    fn second_max(&self) -> i64 {
        match self {
            Self::None | Self::Single(..) => i64::MIN,
            Self::Multiple(e, delta) => max(e.second_max, delta.upd_down) + delta.add,
        }
    }

    fn second_min(&self) -> i64 {
        match self {
            Self::None | Self::Single(..) => i64::MAX,
            Self::Multiple(e, delta) => min(e.second_min, delta.upd_up) + delta.add,
        }
    }

    fn count_min(&self) -> usize {
        match self {
            Self::Multiple(e, _) => e.count_min,
            _ => self.count(),
        }
    }

    fn count_max(&self) -> usize {
        match self {
            Self::Multiple(e, _) => e.count_max,
            _ => self.count(),
        }
    }

    fn get_any_non_extremal(&self) -> Option<i64> {
        let second = self.second_max();
        if second != i64::MIN && second != self.min() {
            Some(second)
        } else {
            None
        }
    }

    fn apply_delta(&mut self, delta: &TreeNodeDelta) {
        match self {
            Self::None => {}
            Self::Single(value, _) => {
                *value = max(min(*value, delta.upd_up), delta.upd_down) + delta.add;
            }
            Self::Multiple(_, self_delta) => {
                self_delta.compose(delta);
                if self.max() == self.min() {
                    *self = Self::Single(self.max(), self.count());
                }
            }
        }
    }

    fn gcd(&self) -> u64 {
        match self {
            TreeNode::None => 0,
            TreeNode::Single(value, _) => value.unsigned_abs(),
            TreeNode::Multiple(..) => {
                let mut g = GCDTracker {
                    last_value: self.get_any_non_extremal().map(|x| x.unsigned_abs()),
                    gcd: self.diff_gcd(),
                };
                g.add_value(self.max().unsigned_abs());
                g.add_value(self.min().unsigned_abs());
                gcd(g.gcd, self.max().unsigned_abs())
            }
        }
    }
}

struct GCDTracker {
    last_value: Option<u64>,
    gcd: u64,
}

impl GCDTracker {
    fn add_value(&mut self, value: u64) {
        match self.last_value {
            None => {
                self.last_value = Some(value);
            }
            Some(v) => {
                self.gcd = gcd(self.gcd, v.abs_diff(value));
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
        let left_min = left.min();
        let right_min = right.min();
        let new_max = max(left_max, right_max);
        let new_min = min(left_min, right_min);
        if new_max == new_min {
            return Self::Single(new_max, new_count);
        }
        let second_max = {
            match left_max.cmp(&right_max) {
                Ordering::Less => max(left_max, right.second_max()),
                Ordering::Equal => max(left.second_max(), right.second_max()),
                Ordering::Greater => max(right_max, left.second_max()),
            }
        };
        let second_min = {
            match left_min.cmp(&right_min) {
                Ordering::Less => min(right_min, left.second_min()),
                Ordering::Equal => min(left.second_min(), right.second_min()),
                Ordering::Greater => min(left_min, right.second_min()),
            }
        };
        Self::Multiple(
            TreeNodeValue {
                count: new_count,
                diff_gcd: {
                    let mut g = GCDTracker {
                        gcd: gcd(left.diff_gcd(), right.diff_gcd()),
                        last_value: None,
                    };
                    if let Some(e) = left.get_any_non_extremal() {
                        g.add_value(e.unsigned_abs());
                    }
                    if let Some(e) = right.get_any_non_extremal() {
                        g.add_value(e.unsigned_abs());
                    }
                    if left_max != new_max && left_max != new_min && left.count() > 0 {
                        g.add_value(left_max.unsigned_abs());
                    }
                    if right_max != new_max && right_max != new_min && right.count() > 0 {
                        g.add_value(right_max.unsigned_abs());
                    }
                    if left_min != new_min && left_min != new_max && left.count() > 0 {
                        g.add_value(left_min.unsigned_abs());
                    }
                    if right_min != new_min && right_min != new_max && right.count() > 0 {
                        g.add_value(right_min.unsigned_abs());
                    }
                    g.gcd
                },
                max: new_max,
                second_max,
                count_max: {
                    match left_max.cmp(&right_max) {
                        Ordering::Less => right.count_max(),
                        Ordering::Equal => left.count_max() + right.count_max(),
                        Ordering::Greater => left.count_max(),
                    }
                },
                min: new_min,
                second_min,
                count_min: {
                    match left_min.cmp(&right_min) {
                        Ordering::Less => left.count_min(),
                        Ordering::Equal => left.count_min() + right.count_min(),
                        Ordering::Greater => right.count_min(),
                    }
                },
                sum: left.sum() + right.sum(),
            },
            TreeNodeDelta::nothing(),
        )
    }

    fn push(&self, child: &mut Self) {
        match self {
            Self::None => panic!("unreachable"),
            Self::Single(value, _) => {
                match child {
                    Self::None => {}
                    _ => {
                        *child = TreeNode::Single(*value, child.count());
                    }
                }
            }
            Self::Multiple(_, delta) => {
                child.apply_delta(delta);
            }
        }
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
            upd_down: i64::MIN,
            add: 0,
        });
    }

    fn tag_condition(&self, node: &TreeNode) -> bool {
        node.second_max() < self.x && self.x < node.max()
    }

    fn break_condition(&self, node: &TreeNode) -> bool {
        node.max() <= self.x
    }
}

struct MaxAssign {
    x: i64,
}

impl SegmentTreeBeatsDelta<TreeNode> for MaxAssign {
    fn apply(&self, node: &mut TreeNode) {
        assert!(self.tag_condition(node));
        node.apply_delta(&TreeNodeDelta {
            upd_up: i64::MAX,
            upd_down: self.x,
            add: 0,
        });
    }

    fn tag_condition(&self, node: &TreeNode) -> bool {
        node.second_min() > self.x && self.x > node.min()
    }

    fn break_condition(&self, node: &TreeNode) -> bool {
        node.min() >= self.x
    }
}

struct AddAssign {
    x: i64,
}

impl SegmentTreeDelta<TreeNode> for AddAssign {
    fn apply(&self, node: &mut TreeNode) {
        node.apply_delta(&TreeNodeDelta {
            upd_up: i64::MAX,
            upd_down: i64::MIN,
            add: self.x,
        });
    }
}

struct Assign {
    x: i64,
}

impl SegmentTreeDelta<TreeNode> for Assign {
    fn apply(&self, node: &mut TreeNode) {
        *node = TreeNode::Single(self.x, node.count());
    }
}

impl From<i64> for TreeNode {
    fn from(value: i64) -> Self {
        Self::Single(value, 1)
    }
}

fn solve(input: &mut Input, _test_case: usize) {
    // test();
    let n = input.read();
    let a = input.read_vec::<i64>(n);
    let mut tree = SegmentTreeBatch::<TreeNode>::from_vec(a);
    let q = input.read();
    for _ in 0..q {
        let (op, left, right): (i32, usize, usize) = input.read();
        let range = (left - 1)..right;
        match op {
            1 => tree.change(range, &MinAssign { x: input.read() }),
            2 => tree.change(range, &MaxAssign { x: input.read() }),
            3 => tree.change(range, &Assign { x: input.read() }),
            4 => tree.change(range, &AddAssign { x: input.read() }),
            5 => {
                out_line!(tree.compute(range).sum());
            }
            6 => {
                out_line!(tree.compute(range).min());
            }
            7 => {
                out_line!(tree.compute(range).max());
            }
            _ => {
                out_line!(tree.compute(range).gcd());
            }
        }
    }
}

impl Default for TreeNode {
    fn default() -> Self {
        Self::Single(0, 1)
    }
}

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
//     fn add(&mut self, range: Range<usize>, value: i64) {
//         for index in range {
//             self.a[index] += value;
//         }
//     }
//
//     fn get_gcd(&self, range: Range<usize>) -> u64 {
//         range
//             .map(|x| self.a[x].unsigned_abs())
//             .fold(0, |a, b| gcd(a, b))
//     }
//
//     fn min(&mut self, range: Range<usize>, value: i64) {
//         for index in range {
//             self.a[index] = min(self.a[index], value);
//         }
//     }
//     fn max(&mut self, range: Range<usize>, value: i64) {
//         for index in range {
//             self.a[index] = max(self.a[index], value);
//         }
//     }
// }

// fn test() {
//     return;
//     loop {
//         let n = 5;
//         let m = 4;
//         let mut t1 = SegmentTreeBatch::<TreeNode>::new(n);
//         let mut t2 = NaiveTree::new(n);
//         let mut rng = rand::thread_rng();
//         let mut qs = String::new();
//         qs.push_str(&format!("{} {}\n", n, m));
//         qs.push_str(&format!("{}\n", &vec!["0".to_string(); n].join(" ")));
//         for _ in 0..m {
//             let op = rng.gen_range(0..5);
//             let left = rng.gen_range(0..n);
//             let right = rng.gen_range(left..n);
//             let range = left..right;
//             if op == 0 {
//                 qs.push_str(&format!("8 {} {}\n", range.start + 1, range.end));
//                 let ans1 = t1.compute(range.clone()).gcd();
//                 let ans2 = t2.get_gcd(range.clone());
//                 if ans1 != ans2 {
//                     eprintln!("{}", qs);
//                     eprintln!("{} {}", ans1, ans2);
//                     panic!();
//                 }
//             } else if op == 1 {
//                 let value = rng.gen_range(1..20);
//                 t2.add(range.clone(), value);
//                 t1.change(range.clone(), &AddAssign { x: value });
//                 qs.push_str(&format!("4 {} {} {}\n", range.start + 1, range.end, value));
//             } else if op == 2 {
//                 qs.push_str(&format!("5 {} {}\n", range.start + 1, range.end));
//                 let ans1 = t1.compute(range.clone()).sum();
//                 let ans2 = t2.get_sum(range.clone());
//                 if ans1 != ans2 {
//                     eprintln!("{}", qs);
//                     eprintln!("{} {}", ans1, ans2);
//                     panic!();
//                 }
//             } else if op == 3 {
//                 let value = rng.gen_range(1..20);
//                 t2.min(range.clone(), value);
//                 t1.change_beats(range.clone(), &MinAssign { x: value });
//                 qs.push_str(&format!("1 {} {} {}\n", range.start + 1, range.end, value));
//             } else {
//                 let value = rng.gen_range(1..20);
//                 t2.max(range.clone(), value);
//                 t1.change_beats(range.clone(), &MaxAssign { x: value });
//                 qs.push_str(&format!("2 {} {} {}\n", range.start + 1, range.end, value));
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

mod tester {
use algo_lib::io::input::Input;
use algo_lib::io::output::{Output, OUTPUT};

fn check(expected: &mut &[u8], actual: &mut &[u8]) -> Result<(), String> {
    let mut expected = Input::new(expected);
    let mut actual = Input::new(actual);
    let mut token_num = 0usize;
    loop {
        let expected_token = expected.next_token();
        let actual_token = actual.next_token();
        if expected_token != actual_token {
            if expected_token.is_none() {
                return Err(format!("Expected has only {} tokens", token_num));
            } else if actual_token.is_none() {
                return Err(format!("Actual has only {} tokens", token_num));
            } else {
                return Err(format!(
                    "Token #{} differs, expected {}, actual {}",
                    token_num,
                    String::from_utf8(expected_token.unwrap()).unwrap(),
                    String::from_utf8(actual_token.unwrap()).unwrap()
                ));
            }
        }
        token_num += 1;
        if actual_token.is_none() {
            break;
        }
    }
    Ok(())
}

static mut OUT: Vec<u8> = Vec::new();

struct WriteDelegate {}

impl std::io::Write for WriteDelegate {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        unsafe {
            OUT.append(&mut Vec::from(buf));
        }
        Ok(buf.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

pub(crate) fn run_tests() -> bool {
    let blue = "\x1B[34m";
    let red = "\x1B[31m";
    let green = "\x1B[32m";
    let yellow = "\x1B[33m";
    let def = "\x1B[0m";
    let time_limit = std::time::Duration::from_millis(4000);
    let mut paths = std::fs::read_dir("./tests/e_minimizirui_pribavlyai_nodirui_iikh_druzya/")
        .unwrap()
        .map(|res| res.unwrap())
        .collect::<Vec<_>>();
    paths.sort_by(|a, b| a.path().cmp(&b.path()));
    let mut test_failed = 0usize;
    let mut test_total = 0usize;
    for path in paths {
        let sub_path = path;
        if sub_path.file_type().unwrap().is_file() {
            let path = sub_path.path();
            match path.extension() {
                None => {}
                Some(extension) => {
                    if extension.to_str() == Some("in") {
                        println!("=====================================================");
                        test_total += 1;
                        let name = path.file_name().unwrap().to_str().unwrap();
                        let name = &name[..name.len() - 3];
                        println!("{}Test {}{}", blue, name, def);
                        println!("{}Input:{}", blue, def);
                        println!("{}", std::fs::read_to_string(&path).unwrap());
                        let expected = match std::fs::read_to_string(
                            path.parent().unwrap().join(format!("{}.out", name)),
                        ) {
                            Ok(res) => Some(res),
                            Err(_) => None,
                        };
                        println!("{}Expected:{}", blue, def);
                        match &expected {
                            None => {
                                println!("{}Not provided{}", yellow, def);
                            }
                            Some(expected) => {
                                println!("{}", expected);
                            }
                        }
                        println!("{}Output:{}", blue, def);
                        match std::panic::catch_unwind(|| {
                            unsafe {
                                OUT.clear();
                            }
                            let mut file = std::fs::File::open(&path).unwrap();
                            let input = Input::new(&mut file);
                            let started = std::time::Instant::now();
                            unsafe {
                                OUTPUT = Some(Output::new(Box::new(WriteDelegate {})));
                            }
                            let is_exhausted = crate::run(input);
                            let res = started.elapsed();
                            let output;
                            unsafe {
                                output = OUT.clone();
                            }
                            println!("{}", String::from_utf8_lossy(&output));
                            (output, res, is_exhausted)
                        }) {
                            Ok((output, duration, is_exhausted)) => {
                                println!(
                                    "{}Time elapsed: {:.3}s{}",
                                    blue,
                                    (duration.as_millis() as f64) / 1000.,
                                    def,
                                );
                                if !is_exhausted {
                                    println!("{}Input not exhausted{}", red, def);
                                }
                                if let Some(expected) = expected {
                                    let mut expected_bytes = expected.as_bytes().clone();
                                    match check(&mut expected_bytes, &mut &output[..]) {
                                        Ok(_) => {}
                                        Err(err) => {
                                            println!(
                                                "{}Verdict: {}Wrong Answer ({}){}",
                                                blue, red, err, def
                                            );
                                            test_failed += 1;
                                            continue;
                                        }
                                    }
                                }
                                if duration > time_limit {
                                    test_failed += 1;
                                    println!("{}Verdict: {}Time Limit{}", blue, red, def);
                                } else {
                                    println!("{}Verdict: {}OK{}", blue, green, def)
                                }
                            }
                            Err(err) => {
                                test_failed += 1;
                                match err.downcast::<&str>() {
                                    Ok(as_string) => println!(
                                        "{}Verdict: {}RuntimeError ({:?}){}",
                                        blue, red, as_string, def
                                    ),
                                    Err(err) => println!(
                                        "{}Verdict: {}RuntimeError ({:?}){}",
                                        blue, red, err, def
                                    ),
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    if test_failed == 0 {
        println!(
            "{}All {}{}{} tests passed{}",
            blue, green, test_total, blue, def
        );
    } else {
        println!(
            "{}{}/{}{} tests failed{}",
            red, test_failed, test_total, blue, def
        );
    }
    test_failed == 0
}
}
#[test]
fn e_minimizirui_pribavlyai_nodirui_iikh_druzya() {
    assert!(tester::run_tests());
}
