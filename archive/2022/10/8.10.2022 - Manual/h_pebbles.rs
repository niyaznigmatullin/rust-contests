//{"name":"h_pebbles","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"h_pebbles"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out, out_line};
use std::iter;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read();
    let a = input.read_vec::<u32>(n);
    let mut d = a
        .iter()
        .zip(iter::once(&0).chain(a.iter()))
        .map(|(x, y)| *x - *y)
        .collect::<Vec<_>>();
    d.reverse();
    let mut xor = 0;
    for i in (0..d.len()).step_by(2) {
        xor ^= d[i];
    }
    out_line!(if xor == 0 { "NIE" } else { "TAK" });
}

pub(crate) fn run(mut input: Input) -> bool {
    let tests = input.read();
    for test_case in 1..=tests {
        solve(&mut input, test_case);
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
