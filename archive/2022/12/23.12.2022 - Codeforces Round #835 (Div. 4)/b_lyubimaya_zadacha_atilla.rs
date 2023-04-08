//{"name":"B. Любимая задача Atilla","group":"Codeforces - Codeforces Round #835 (Div. 4)","url":"https://codeforces.com/contest/1760/problem/B","interactive":false,"timeLimit":1000,"tests":[{"input":"5\n1\na\n4\ndown\n10\ncodeforces\n3\nbcf\n5\nzzzzz\n","output":"1\n23\n19\n6\n26\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BLyubimayaZadachaAtilla"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let (_, s): (String, String) = input.read();
    let c = s.as_bytes().into_iter().map(|z| *z - b'a').max().unwrap();
    out_line!(c + 1);
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
