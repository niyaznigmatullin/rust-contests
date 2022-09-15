//{"name":"C. Бесконечная замена","group":"Codeforces - Codeforces Round #786 (Div. 3)","url":"https://codeforces.com/contest/1674/problem/C","interactive":false,"timeLimit":2000,"tests":[{"input":"3\naaaa\na\naa\nabc\na\nb\n","output":"1\n-1\n2\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CBeskonechnayaZamena"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let s = input.read::<String>();
    let t = input.read::<String>();
    if t == "a" {
        out_line!(1);
    } else if t.contains("a") {
        out_line!(-1);
    } else {
        out_line!(1u64 << s.len());
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


//START MAIN
mod tester;

fn main() {
    tester::run_tests();
}
//END MAIN
