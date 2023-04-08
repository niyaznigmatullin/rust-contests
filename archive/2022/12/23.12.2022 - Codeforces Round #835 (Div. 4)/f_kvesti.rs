//{"name":"F. Квесты","group":"Codeforces - Codeforces Round #835 (Div. 4)","url":"https://codeforces.com/contest/1760/problem/F","interactive":false,"timeLimit":3000,"tests":[{"input":"6\n2 5 4\n1 2\n2 20 10\n100 10\n3 100 3\n7 2 6\n4 20 3\n4 5 6 7\n4 100000000000 2022\n8217734 927368 26389746 627896974\n2 20 4\n5 1\n","output":"2\nInfinity\nImpossible\n1\n12\n0\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"FKvesti"}}}

use std::cmp::min;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let (n, c, d) = input.read();
    let mut a = input.read_vec::<i64>(n);
    a.sort_by_key(|x| -x);
    if a[0] * (d as i64) < c {
        out_line!("Impossible");
        return;
    }
    if a.iter().take(min(d, n)).sum::<i64>() >= c {
        out_line!("Infinity");
        return;
    }
    for i in 1..a.len() {
        a[i] += a[i - 1];
    }
    let mut left = 0;
    let mut right = d + 1;
    while left < right - 1 {
        let mid = (left + right) / 2;
        let block_size = mid + 1;
        let take_count = min(block_size, n);
        let mut all = a[take_count - 1] * ((d / block_size) as i64);
        let left_to_take = min(d % block_size, n);
        if left_to_take > 0 {
            all += a[left_to_take - 1];
        }
        if all >= c {
            left = mid;
        } else {
            right = mid;
        }
    }
    out_line!(left);
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
