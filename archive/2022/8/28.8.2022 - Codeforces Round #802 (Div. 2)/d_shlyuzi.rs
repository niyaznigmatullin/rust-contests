//{"name":"D. Шлюзы","group":"Codeforces - Codeforces Round #802 (Div. 2)","url":"https://codeforces.com/contest/1700/problem/D","interactive":false,"timeLimit":2000,"tests":[{"input":"5\n4 1 5 4 1\n6\n1\n6\n2\n3\n4\n5\n","output":"-1\n3\n-1\n-1\n4\n3\n"},{"input":"5\n4 4 4 4 4\n6\n1\n3\n6\n5\n2\n4\n","output":"-1\n-1\n4\n4\n-1\n5\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DShlyuzi"}}}

use std::cmp::max;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read();
    let mut v = input.read_vec::<u64>(n);
    for i in 1..n {
        v[i] += v[i - 1];
    }
    let mut min_t = 0;
    for i in 0..n {
        min_t = max(min_t, (v[i] + i as u64) / (i as u64 + 1));
    }
    let q = input.read();
    for _ in 0..q {
        let t: u64 = input.read();
        if t < min_t {
            out_line!(-1);
        } else {
            out_line!((v[n - 1] + t - 1) / t);
        }
    }
}

pub(crate) fn run(mut input: Input) -> bool {
    solve(&mut input, 1);
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
