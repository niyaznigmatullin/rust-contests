//{"name":"A2: Consecutive Cuts - Chapter 2","group":"Meta Coding Competitions - Meta Hacker Cup 2022 Round 1","url":"https://www.facebook.com/codingcompetitions/hacker-cup/2022/round-1/problems/A2","interactive":false,"timeLimit":360000,"tests":[{"input":"4\n5 1\n5 1 2 2 3\n2 2 3 5 1\n4 10\n3 1 4 2\n1 2 3 4\n4 0\n3 1 4 2\n2 3 1 4\n5 3\n10 10 9 10 9\n10 10 10 9 9\n","output":"Case #1: YES\nCase #2: NO\nCase #3: NO\nCase #4: NO\n"}],"testType":"multiNumber","input":{"type":"regex","fileName":null,"pattern":"consecutive_cuts_chapter__.*input[.]txt"},"output":{"type":"file","fileName":"consecutive_cuts_chapter__output.txt","pattern":null},"languages":{"java":{"taskClass":"A2ConsecutiveCutsChapter2"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::strings::prefix_function::prefix_function;
use algo_lib::{out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let (n, k): (usize, usize) = input.read();
    let a = input.read_vec(n);
    let b = input.read_vec(n);
    out!(format!("Case #{}: ", _test_case));
    let from = k as usize;
    let to = k as i64 * (n - 1) as i64 + 1;
    let dist = if to >= (from + n) as i64 {
        n
    } else {
        to as usize - from
    };
    let from = from % n;
    let mut s = vec![0; 4 * n + 1];
    s[0..n].copy_from_slice(&b);
    s[n] = -1;
    s[n + 1..2 * n + 1].copy_from_slice(&a);
    s[2 * n + 1..3 * n + 1].copy_from_slice(&a);
    s[3 * n + 1..4 * n + 1].copy_from_slice(&a);
    let p = prefix_function(&s);
    out_line!(if *p[n + 1 + from + n - 1..n + 1 + from + dist + n - 1]
        .iter()
        .max()
        .unwrap()
        == n
    {
        "YES"
    } else {
        "NO"
    });
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

//START MAIN
mod tester;

fn main() {
    tester::run_tests();
}
//END MAIN
