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
    let time_limit = std::time::Duration::from_millis(2000);
    let mut paths = std::fs::read_dir("./tests/kth_element/")
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
fn kth_element() {
    assert!(tester::run_tests());
}
