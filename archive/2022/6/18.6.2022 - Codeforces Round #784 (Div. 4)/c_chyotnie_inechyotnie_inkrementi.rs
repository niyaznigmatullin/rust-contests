//{"name":"C. Чётные и нечётные инкременты","group":"Codeforces - Codeforces Round #784 (Div. 4)","url":"https://codeforces.com/contest/1669/problem/C","interactive":false,"timeLimit":1000,"tests":[{"input":"4\n3\n1 2 1\n4\n2 2 2 3\n4\n2 2 2 2\n5\n1000 1 1000 1 1000\n","output":"YES\nNO\nYES\nYES\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CChyotnieINechyotnieInkrementi"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read();
    let a = input.read_vec::<u32>(n);
    let mut ans = true;
    for i in 0..n {
        if a[i] % 2 != a[i % 2] % 2 {
            ans = false;
            break;
        }
    }
    out_line!(if ans { "YES" } else { "NO" });
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
