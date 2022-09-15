//{"name":"A. Выведите пьедестал (логотип Codeforces?)","group":"Codeforces - Codeforces Round #797 (Div. 3)","url":"https://codeforces.com/contest/1690/problem/A","interactive":false,"timeLimit":1000,"tests":[{"input":"6\n11\n6\n10\n100000\n7\n8\n","output":"4 5 2\n2 3 1\n4 5 1\n33334 33335 33331\n2 4 1\n3 4 1\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"AViveditePedestalLogotipCodeforces"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let n: u32 = input.read();
    let h3 = (n - 3) / 3;
    let h2 = (n - h3 - 1) / 2;
    let h1 = n - h3 - h2;
    out_line!(h2, h1, h3);
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
