//{"name":"C. Прыжки по плиткам","group":"Codeforces - Codeforces Round #820 (Div. 3)","url":"https://codeforces.com/contest/1729/problem/C","interactive":false,"timeLimit":1000,"tests":[{"input":"6\nlogic\ncodeforces\nbca\naaaaaaaaaaa\nadbaadabad\nto\n","output":"9 4\n1 4 3 5\n16 10\n1 8 3 4 9 5 2 6 7 10\n1 2\n1 3\n0 11\n1 8 10 4 3 5 7 2 9 6 11\n3 10\n1 9 5 4 7 3 8 6 2 10\n5 2\n1 2\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CPrizhkiPoPlitkam"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out, out_line};
use std::cmp::{max, min};
use std::iter;

fn solve(input: &mut Input, _test_case: usize) {
    let s = input.read::<String>().into_bytes();
    let first = s[0];
    let last = s.last().copied().unwrap();
    let mut ans = Vec::new();
    for letter in min(first, last)..=max(first, last) {
        for index in 1..s.len() - 1 {
            if s[index] == letter {
                ans.push(index);
            }
        }
    }
    if first > last {
        ans.reverse();
    }
    let ans = iter::once(0)
        .chain(ans.into_iter())
        .chain(iter::once(s.len() - 1))
        .map(|x| (x + 1).to_string())
        .collect::<Vec<_>>();
    let cost = first.abs_diff(last);
    out_line!(cost, ans.len());
    out_line!(ans.join(" "));
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
