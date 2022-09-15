//{"name":"A. Разноцветные шары: перезагрузка","group":"Codeforces - Educational Codeforces Round 135 (Rated for Div. 2)","url":"https://codeforces.com/contest/1728/problem/A","interactive":false,"timeLimit":2000,"tests":[{"input":"3\n3\n1 1 1\n1\n9\n2\n4 7\n","output":"3\n1\n2\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"ARaznotsvetnieShariPerezagruzka"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read();
    let a = input.read_vec::<i32>(n);
    out_line!(a.into_iter().zip(0..n).max_by_key(|(v, i)| *v).unwrap().1 + 1);
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
