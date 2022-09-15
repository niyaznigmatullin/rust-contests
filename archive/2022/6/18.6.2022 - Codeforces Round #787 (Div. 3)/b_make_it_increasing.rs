//{"name":"B. Make It Increasing","group":"Codeforces - Codeforces Round #787 (Div. 3)","url":"https://codeforces.com/contest/1675/problem/B","interactive":false,"timeLimit":2000,"tests":[{"input":"7\n3\n3 6 5\n4\n5 3 2 1\n5\n1 2 3 4 5\n1\n1000000000\n4\n2 8 7 5\n5\n8 26 5 21 10\n2\n5 14\n","output":"2\n-1\n0\n0\n4\n11\n0\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BMakeItIncreasing"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read();
    let mut a = input.read_vec::<u32>(n);
    let mut ans = 0;
    for i in (0..n - 1).rev() {
        while a[i] > 0 && a[i] >= a[i + 1] {
            a[i] /= 2;
            ans += 1;
        }
        if a[i] >= a[i + 1] {
            ans = -1;
            break;
        }
    }
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
