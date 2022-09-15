//{"name":"A. Мадока и странные мысли","group":"Codeforces - Codeforces Round #818 (Div. 2)","url":"https://codeforces.com/contest/1717/problem/A","interactive":false,"timeLimit":1000,"tests":[{"input":"6\n1\n2\n3\n4\n5\n100000000\n","output":"1\n4\n7\n10\n11\n266666666\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"AMadokaIStrannieMisli"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let n: i32 = input.read();
    let ans = n + (n / 2 + n / 3) * 2;
    out_line!(ans);
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
