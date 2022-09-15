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

    fn push(&self, child: &mut Self) {
        child.add += self.add;
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
            tree.change(l..r, &ModOperation { x });
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
    let mut paths = std::fs::read_dir("./tests/a_rebenok_iposledovatelnost/")
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
fn a_rebenok_iposledovatelnost() {
    assert!(tester::run_tests());
}
