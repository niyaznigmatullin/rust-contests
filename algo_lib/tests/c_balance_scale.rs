//{"name":"C: Balance Scale","group":"Meta Coding Competitions - Meta Hacker Cup 2022 Round 2","url":"https://www.facebook.com/codingcompetitions/hacker-cup/2022/round-2/problems/C","interactive":false,"timeLimit":360000,"tests":[{"input":"6\n5 1\n1 3000\n1 2000\n1 1000\n1 2000\n1 1000\n5 2\n1 3000\n1 2000\n1 1000\n1 2000\n1 1000\n2 10\n10 1\n10 2\n5 2\n2 50\n1 40\n1 50\n1 60\n3 50\n4 2993\n3000 999999999\n2995 1000000000\n1552 888888888\n1336 999999999\n3 1\n1 10\n2 9\n1 11\n","output":"Case #1: 800000006\nCase #2: 200000002\nCase #3: 0\nCase #4: 208333335\nCase #5: 590307096\nCase #6: 333333336\n"}],"testType":"multiNumber","input":{"type":"regex","fileName":null,"pattern":"balance_scale_.*input[.]txt"},"output":{"type":"file","fileName":"balance_scale_output.txt","pattern":null},"languages":{"java":{"taskClass":"CBalanceScale"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::math::modular::factorial::Factorial;
use algo_lib::math::modular::primitive::{Modular, ModularType};
use algo_lib::{out, out_line};
use std::cmp::min;

type Mod = ModularType<1000000007>;

fn solve(input: &mut Input, _test_case: usize) {
    out!(format!("Case #{}: ", _test_case));
    let (n, k): (usize, usize) = input.read();
    let k = k + 1;
    let a = input.read_vec::<(usize, i32)>(n);
    let count_all = a.iter().map(|(count, _)| *count).sum::<usize>();
    let factorials = Factorial::new(count_all + 1);
    let mut ways = Mod::from(0);
    let count_need = a[0].0;
    let count_same_weight = a
        .iter()
        .filter(|(_, w)| *w == a[0].1)
        .map(|(count, _)| *count)
        .sum::<usize>();
    let count_smaller_weight = a
        .iter()
        .filter(|(_, w)| *w < a[0].1)
        .map(|(count, _)| *count)
        .sum::<usize>();
    for k1 in 1..=min(k, count_same_weight) {
        ways += Mod::from(count_need)
            * factorials.combinations(count_same_weight - 1, k1 - 1)
            * factorials.factorial(k1 - 1)
            * factorials.combinations(count_smaller_weight, k - k1)
            * factorials.factorial(k - k1)
            * factorials.combinations(k, k1);
    }
    ways *= factorials.inverse_combinations(count_all, k)
        * factorials.inverse_factorial(k);
    out_line!(ways);
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
    let time_limit = std::time::Duration::from_millis(360000);
    let mut paths = std::fs::read_dir("./tests/c_balance_scale/")
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
fn c_balance_scale() {
    assert!(tester::run_tests());
}
