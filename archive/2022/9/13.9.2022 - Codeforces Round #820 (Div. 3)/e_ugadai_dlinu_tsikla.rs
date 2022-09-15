//{"name":"E. Угадай длину цикла","group":"Codeforces - Codeforces Round #820 (Div. 3)","url":"https://codeforces.com/contest/1729/problem/E","interactive":true,"timeLimit":1000,"tests":[{"input":"1\n\n2\n\n-1\n","output":"? 1 2\n\n? 1 3\n\n? 1 4\n\n! 3\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"EUgadaiDlinuTsikla"}}}

use std::cmp::max;
use std::io::{stdout, Write};
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out, out_line};

fn ask(input: &mut Input, x: i64, y: i64) -> i64 {
    println!("? {} {}", x, y);
    let _ = stdout().flush();
    input.read()
}

fn solve(input: &mut Input, _test_case: usize) {
    let mut best = 0;
    for i in 2..22 {
        let d1 = ask(input,1, i);
        let d2 = ask(input, i, 1);
        if d1 == -1 || d2 == -1 {
            println!("! {}", i - 1);
            let _ = stdout().flush();
            return;
        }
        if d1 != d2 {
            println!("! {}", d1 + d2);
            let _ = stdout().flush();
            return;
        }
        best = max(best, d1);
    }
    println!("! {}", 2 * best);
}

pub(crate) fn run(mut input: Input) -> bool {
    solve(&mut input, 1);
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
