//{"name":"D. A-B-C Сортировка","group":"Codeforces - Codeforces Round #786 (Div. 3)","url":"https://codeforces.com/contest/1674/problem/D","interactive":false,"timeLimit":2000,"tests":[{"input":"3\n4\n3 1 5 3\n3\n3 2 1\n1\n7331\n","output":"YES\nNO\nYES\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DABCSortirovka"}}}

use std::mem::swap;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read();
    let mut a = input.read_vec::<u32>(n);
    for i in (n % 2..n).step_by(2) {
        if a[i] > a[i + 1] {
            let t = a[i];
            a[i] = a[i + 1];
            a[i + 1] = t;
        }
    }
    let mut ans = true;
    for i in 0..n - 1 {
        if a[i] > a[i + 1] {
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
