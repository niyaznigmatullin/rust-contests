//{"name":"C. Преимущество","group":"Codeforces - Codeforces Round #835 (Div. 4)","url":"https://codeforces.com/contest/1760/problem/C","interactive":false,"timeLimit":2000,"tests":[{"input":"5\n4\n4 7 3 5\n2\n1 2\n5\n1 2 3 4 5\n3\n4 9 4\n4\n4 4 4 4\n","output":"-3 2 -4 -2\n-1 1\n-4 -3 -2 -1 1\n-5 5 -5\n0 0 0 0\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CPreimushchestvo"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read();
    let a = input.read_vec(n);
    let mut max1 = 0;
    let mut max2 = 0;
    for &x in &a {
        if max1 < x {
            max2 = max1;
            max1 = x;
        } else if max2 < x {
            max2 = x;
        }
    }
    out_line!(a.into_iter()
        .map(|x| x - if x == max1 { max2 } else { max1 }).collect::<Vec<_>>());
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
