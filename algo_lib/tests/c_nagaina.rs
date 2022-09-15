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

    fn push(&self, left: &mut Self) {
        left.upd = min(left.upd, self.upd);
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
        self.for_sum.change(range, &MinWith { x: value });
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
    let mut paths = std::fs::read_dir("./tests/c_nagaina/")
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
fn c_nagaina() {
    assert!(tester::run_tests());
}
