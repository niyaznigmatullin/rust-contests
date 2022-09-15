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
            1 => tree.change(range, &MinAssign { x: input.read() }),
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
    let time_limit = std::time::Duration::from_millis(3000);
    let mut paths = std::fs::read_dir("./tests/d_minimizirui_pribavlyai/")
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
fn d_minimizirui_pribavlyai() {
    assert!(tester::run_tests());
}
