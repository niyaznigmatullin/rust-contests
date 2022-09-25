//{"name":"D2: Work-Life Balance - Chapter 2","group":"Meta Coding Competitions - Meta Hacker Cup 2022 Round 2","url":"https://www.facebook.com/codingcompetitions/hacker-cup/2022/round-2/problems/D2","interactive":false,"timeLimit":360000,"tests":[{"input":"3\n2 3\n1 2\n2 1 1\n1 2 1\n1 2 1\n4 3\n1 1 1 2\n1 2 2\n2 2 2\n4 1 2\n8 5\n1 1 1 1 2 2 2 2\n5 2 4\n7 2 3\n6 2 5\n1 2 4\n3 2 4\n","output":"Case #1: -2\nCase #2: 0\nCase #3: 16\n"}],"testType":"multiNumber","input":{"type":"regex","fileName":null,"pattern":"worklife_balance_chapter__.*input[.]txt"},"output":{"type":"file","fileName":"worklife_balance_chapter__output.txt","pattern":null},"languages":{"java":{"taskClass":"D2WorkLifeBalanceChapter2"}}}

use algo_lib::datastructures::fenwick::kth::FenwickKth;
use algo_lib::datastructures::segment_tree::common::SegmentTreeNode;
use algo_lib::datastructures::segment_tree::simple::SegmentTree;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out, out_line};
use std::cmp::{max, min};

#[derive(Copy, Clone)]
struct InversionNode {
    count1: usize,
    count2: usize,
    count12: i64,
    count21: i64,
}

impl SegmentTreeNode for InversionNode {
    fn neutral() -> Self {
        Self {
            count1: 0,
            count2: 0,
            count12: 0,
            count21: 0,
        }
    }

    fn join(left: &Self, right: &Self) -> Self {
        Self {
            count1: left.count1 + right.count1,
            count2: left.count2 + right.count2,
            count12: left.count12 + right.count12 + left.count1 as i64 * right.count2 as i64,
            count21: left.count21 + right.count21 + left.count2 as i64 * right.count1 as i64,
        }
    }
}

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read();
    let m = input.read();
    let mut a = input.read_vec::<usize>(n);
    let mut finder = vec![FenwickKth::new(n), FenwickKth::new(n)];
    let nodes = vec![
        InversionNode {
            count1: 1,
            count2: 0,
            count12: 0,
            count21: 0,
        },
        InversionNode {
            count1: 0,
            count2: 1,
            count12: 0,
            count21: 0,
        },
    ];
    let mut inversions = SegmentTree::from_vec(a.iter().map(|x| nodes[*x - 1]).collect());
    let mut sum = 0;
    for i in 0..n {
        finder[a[i] - 1].add(i, 1);
        inversions.set(i, nodes[a[i] - 1]);
        sum += a[i];
    }
    let mut ans = 0;
    for _ in 0..m {
        let (x, y, z): (usize, usize, usize) = input.read();
        let x = x - 1;
        sum -= a[x];
        finder[a[x] - 1].remove(x, 1);
        a[x] = y;
        inversions.set(x, nodes[a[x] - 1]);
        finder[a[x] - 1].add(x, 1);
        sum += a[x];
        let result = calculate(n, &finder, &inversions, sum, z);
        ans += result;
    }
    out_line!(format!("Case #{}: {}", _test_case, ans));
}

fn calculate(
    n: usize,
    finder: &Vec<FenwickKth>,
    inversions: &SegmentTree<InversionNode>,
    sum: usize,
    z: usize,
) -> i64 {
    if sum % 2 != 0 {
        return -1;
    }
    let half = sum / 2;
    if half < max(z, n - z) || half > 2 * min(z, n - z) {
        return -1;
    }
    let twos = half - z;
    let ones = 2 * z - half;
    if ones == 0 {
        return match finder[1].get_kth_pos(twos) {
            Some(pos2) => inversions.compute(0..(pos2 + 1)).count12,
            _ => -1,
        };
    }
    if twos == 0 {
        return match finder[0].get_kth_pos(ones) {
            Some(pos1) => inversions.compute(0..(pos1 + 1)).count21,
            _ => -1,
        };
    }
    let pos1 = finder[0].get_kth_pos(ones);
    let pos2 = finder[1].get_kth_pos(twos);
    if pos1.is_none() || pos2.is_none() {
        return -1;
    }
    let pos1 = pos1.unwrap();
    let pos2 = pos2.unwrap();
    if pos1 < z && pos2 < z {
        return 0;
    }
    if pos1 < pos2 {
        inversions.compute((pos1 + 1)..(pos2 + 1)).count12
    } else {
        inversions.compute((pos2 + 1)..(pos1 + 1)).count21
    }
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
    let mut paths = std::fs::read_dir("./tests/d2_work_life_balance_chapter2/")
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
fn d2_work_life_balance_chapter2() {
    assert!(tester::run_tests());
}
