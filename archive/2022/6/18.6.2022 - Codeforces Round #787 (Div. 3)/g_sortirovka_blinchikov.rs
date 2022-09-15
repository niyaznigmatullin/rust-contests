//{"name":"G. Сортировка блинчиков","group":"Codeforces - Codeforces Round #787 (Div. 3)","url":"https://codeforces.com/contest/1675/problem/G","interactive":false,"timeLimit":2000,"tests":[{"input":"6 19\n5 3 2 3 3 3\n","output":"2\n"},{"input":"3 6\n3 2 1\n","output":"0\n"},{"input":"3 6\n2 1 3\n","output":"1\n"},{"input":"6 19\n3 4 3 3 5 1\n","output":"3\n"},{"input":"10 1\n0 0 0 0 0 0 0 0 0 1\n","output":"9\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"GSortirovkaBlinchikov"}}}

use std::cmp::min;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read();
    let m: usize = input.read();
    let a = input.read_vec::<i32>(n);
    let mut f = vec![vec![i32::MAX; m + 1]; m + 1];
    f[0][m] = 0;
    let mut sum_prev = 0;
    for &cur in a.iter() {
        let mut nf = vec![vec![i32::MAX; m + 1]; m + 1];
        sum_prev += cur;
        for sum in 0..=m {
            for last in 0..=m {
                let value = f[sum][last];
                if value == i32::MAX {
                    continue;
                }
                // out_line!(i, sum, last, value);
                for next in 0..=min(m - sum, last) {
                    nf[sum + next][next] = min(nf[sum + next][next], value + (sum_prev - (sum + next) as i32).abs());
                }
            }
        }
        f = nf;
    }
    let mut ans = i32::MAX;
    for last in 0..=m {
        ans = min(ans, f[m][last]);
    }
    out_line!(ans);
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
