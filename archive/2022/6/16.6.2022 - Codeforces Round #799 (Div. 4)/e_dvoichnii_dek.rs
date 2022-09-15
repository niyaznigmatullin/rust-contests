//{"name":"E. Двоичный дек","group":"Codeforces - Codeforces Round #799 (Div. 4)","url":"https://codeforces.com/contest/1692/problem/E","interactive":false,"timeLimit":2000,"tests":[{"input":"7\n3 1\n1 0 0\n3 1\n1 1 0\n9 3\n0 1 0 1 1 1 0 0 1\n6 4\n1 1 1 1 1 1\n5 1\n0 0 1 1 0\n16 2\n1 1 0 0 1 0 0 1 1 0 0 0 0 0 1 1\n6 3\n1 0 1 0 0 0\n","output":"0\n1\n3\n2\n2\n7\n-1\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"EDvoichniiDek"}}}

use std::cmp::min;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let mut sum = 0;
    let n = input.read();
    let s = input.read();
    let a: Vec<_> = (0..n).map(|_| input.read::<usize>()).collect();
    let mut right = 0;
    let mut min_delete = n + 1;
    for left in 0..n {
        while right < n && sum + a[right] <= s {
            sum += a[right];
            right += 1;
        }
        if sum == s {
            min_delete = min(min_delete, n - (right - left));
        }
        sum -= a[left];
    }
    if min_delete > n {
        out_line!("-1");
    } else {
        out_line!(min_delete);
    }
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
