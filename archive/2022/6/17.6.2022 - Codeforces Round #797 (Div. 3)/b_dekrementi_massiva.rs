//{"name":"B. Декременты массива","group":"Codeforces - Codeforces Round #797 (Div. 3)","url":"https://codeforces.com/contest/1690/problem/B","interactive":false,"timeLimit":1000,"tests":[{"input":"6\n4\n3 5 4 1\n1 3 2 0\n3\n1 2 1\n0 1 0\n4\n5 3 7 2\n1 1 1 1\n5\n1 2 3 4 5\n1 2 3 4 6\n1\n8\n0\n1\n4\n6\n","output":"YES\nYES\nNO\nNO\nYES\nNO\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BDekrementiMassiva"}}}

use std::cmp::max;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read();
    let a = input.read_vec::<u32>(n);
    let b = input.read_vec::<u32>(n);
    let mut dif = None;
    let mut dif_at_least = 0;
    for i in 0..n {
        if a[i] < b[i] {
            out_line!("NO");
            return;
        }
        if b[i] == 0 {
            dif_at_least = max(dif_at_least, a[i]);
        } else {
            if let Some(d) = dif {
                if d != a[i] - b[i] {
                    out_line!("NO");
                    return;
                }
            } else {
                dif = Some(a[i] - b[i]);
            }
        }
    }
    if let Some(d) = dif {
        if dif_at_least > d {
            out_line!("NO");
            return;
        }
    }
    out_line!("YES");
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
