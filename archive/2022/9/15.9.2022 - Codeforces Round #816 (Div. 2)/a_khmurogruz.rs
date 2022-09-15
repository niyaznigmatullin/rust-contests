//{"name":"A. Хмурогруз","group":"Codeforces - Codeforces Round #816 (Div. 2)","url":"https://codeforces.com/contest/1715/problem/A?locale=ru","interactive":false,"timeLimit":1000,"tests":[{"input":"7\n7 5\n5 7\n1 1\n100000 100000\n57 228\n1 5\n5 1\n","output":"15\n15\n0\n299998\n340\n5\n5\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"AKhmurogruz"}}}

use std::cmp::{max, min};
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let n: i32 = input.read();
    let m: i32 = input.read();
    out_line!(if n == 1 && m == 1 {
        0
    } else if min(n, m) == 1 {
        max(n, m)
    } else {
        (n + m - 2) * 2 - (max(n, m) - 2)
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
