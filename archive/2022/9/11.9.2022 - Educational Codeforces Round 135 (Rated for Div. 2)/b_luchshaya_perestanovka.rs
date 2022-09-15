//{"name":"B. Лучшая перестановка","group":"Codeforces - Educational Codeforces Round 135 (Rated for Div. 2)","url":"https://codeforces.com/contest/1728/problem/B","interactive":false,"timeLimit":2000,"tests":[{"input":"3\n4\n5\n6\n","output":"2 1 3 4\n1 2 3 4 5\n4 5 1 2 3 6\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BLuchshayaPerestanovka"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out, out_line};
use std::io::Read;
use std::{iter, mem};

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read();
    let mut a = (1..(n - 1)).rev().collect::<Vec<_>>();
    a.push(n - 1);
    a.push(n);
    if n % 2 == 1 {
        let t = a[0];
        a[0] = a[1];
        a[1] = t;
    }
    out_line!(a
        .into_iter()
        .map(|x| x.to_string())
        .collect::<Vec<_>>()
        .join(" "));
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
