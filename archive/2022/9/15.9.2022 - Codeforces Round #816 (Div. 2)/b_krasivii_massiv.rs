//{"name":"B. Красивый массив","group":"Codeforces - Codeforces Round #816 (Div. 2)","url":"https://codeforces.com/contest/1715/problem/B?locale=ru","interactive":false,"timeLimit":1000,"tests":[{"input":"8\n1 6 3 100\n3 6 3 12\n3 6 3 19\n5 4 7 38\n5 4 7 80\n99978 1000000000 100000000 1000000000000000000\n1 1 0 0\n4 1000000000 1000000000 1000000000000000000\n","output":"-1\n-1\n0 0 19\n0 3 3 3 29\n-1\n-1\n0\n0 0 0 1000000000000000000\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BKrasiviiMassiv"}}}

use std::cmp::min;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read();
    let k: i64 = input.read();
    let b: i64 = input.read();
    let s: i64 = input.read();
    if s < b * k || s - b * k > (k - 1) * n as i64 {
        out_line!(-1);
        return;
    }
    let mut ans = vec![0; n];
    ans[0] = b * k;
    let mut remainder = s - b * k;
    for i in 0..n {
        let put = min(remainder, k - 1);
        remainder -= put;
        ans[i] += put;
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
