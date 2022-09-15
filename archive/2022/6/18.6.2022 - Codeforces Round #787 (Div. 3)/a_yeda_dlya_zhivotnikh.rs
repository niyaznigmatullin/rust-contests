//{"name":"A. Еда для животных","group":"Codeforces - Codeforces Round #787 (Div. 3)","url":"https://codeforces.com/contest/1675/problem/A","interactive":false,"timeLimit":1000,"tests":[{"input":"7\n1 1 4 2 3\n0 0 0 0 0\n5 5 0 4 6\n1 1 1 1 1\n50000000 50000000 100000000 100000000 100000000\n0 0 0 100000000 100000000\n1 3 2 2 5\n","output":"YES\nYES\nNO\nYES\nYES\nNO\nNO\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"AYedaDlyaZhivotnikh"}}}

use std::cmp::min;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let (mut a, mut b, c, x, y) : (i32, i32, i32, i32, i32) = input.read();
    a = min(a, x);
    b = min(b, y);
    out_line!(if a + b + c < x + y {
        "NO"
    } else {
        "YES"
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
