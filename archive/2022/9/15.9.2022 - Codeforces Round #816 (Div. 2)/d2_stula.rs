//{"name":"D. 2+ стула","group":"Codeforces - Codeforces Round #816 (Div. 2)","url":"https://codeforces.com/contest/1715/problem/D?locale=ru","interactive":false,"timeLimit":1500,"tests":[{"input":"4 3\n1 2 3\n1 3 2\n4 1 2\n","output":"0 3 2 2\n"},{"input":"1 0\n","output":"0\n"},{"input":"2 1\n1 1 1073741823\n","output":"1073741823 0\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"D2Stula"}}}

use std::ops::Not;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read();
    let q = input.read();
    let mut a = vec![(1 << 30) - 1; n];
    let mut pairs = vec![Vec::new(); n];
    for _ in 0..q {
        let i = input.read::<usize>() - 1;
        let j = input.read::<usize>() - 1;
        let x: i32 = input.read();
        a[i] &= x;
        a[j] &= x;
        pairs[i].push((j, x));
        pairs[j].push((i, x));
    }
    for i in 0..n {
        let mut mask = 0;
        for &z in &pairs[i] {
            if z.0 == i {
                mask |= z.1;
            } else {
                mask |= z.1 & a[z.0].not();
            }
        }
        a[i] = mask;
    }
    out_line!(a);
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
