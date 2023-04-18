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
    let mut paths = std::fs::read_dir("./tests/find2d/")
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
fn find2d() {
    assert!(tester::run_tests());
}
