//{"name":"D. Часы","group":"Codeforces - Codeforces Round #799 (Div. 4)","url":"https://codeforces.com/contest/1692/problem/D","interactive":false,"timeLimit":1000,"tests":[{"input":"6\n03:12 360\n00:00 1\n13:22 2\n15:15 10\n11:11 1440\n22:30 27\n","output":"1\n16\n10\n0\n1\n1\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DChasi"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let mut s = decode(&input.read::<String>());
    let x: usize = input.read();
    let mut used = vec![false; 60 * 24];
    let mut ans = 0;
    while !used[s] {
        used[s] = true;
        if is_palindrome(&encode(s)) {
            ans += 1;
        }
        s = (s + x) % used.len();
    }
    out_line!(ans);
}

fn is_palindrome(s: &str) -> bool {
    s.chars().rev().collect::<String>() == s
}

fn decode(s: &str) -> usize {
    let h: usize = s[0..2].parse().unwrap();
    let m: usize = s[3..5].parse().unwrap();
    h * 60 + m
}

fn encode(time: usize) -> String {
    format!("{:02}:{:02}", time / 60, time % 60)
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
