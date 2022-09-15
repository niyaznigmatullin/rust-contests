//{"name":"D. Черно-белая полоса","group":"Codeforces - Codeforces Round #797 (Div. 3)","url":"https://codeforces.com/contest/1690/problem/D","interactive":false,"timeLimit":2000,"tests":[{"input":"4\n5 3\nBBWBW\n5 5\nBBWBW\n5 1\nBBWBW\n1 1\nW\n","output":"1\n2\n0\n1\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DChernoBelayaPolosa"}}}

use std::cmp::min;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read();
    let k = input.read();
    let mut s : Vec<_> = input.read::<String>()
        .into_bytes()
        .into_iter()
        .map(|x| if x == b'W' { 1 } else { 0 })
        .collect();
    for i in 1..n {
        s[i] += s[i - 1];
    }
    let mut ans = k;
    for i in k - 1..n {
        let mut got = s[i];
        if i >= k {
            got -= s[i - k];
        }
        ans = min(ans, got);
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
