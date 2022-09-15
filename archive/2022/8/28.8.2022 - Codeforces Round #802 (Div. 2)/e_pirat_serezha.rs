//{"name":"E. Пират Сережа","group":"Codeforces - Codeforces Round #802 (Div. 2)","url":"https://codeforces.com/contest/1700/problem/E","interactive":false,"timeLimit":2000,"tests":[{"input":"3 3\n2 1 3\n6 7 4\n9 8 5\n","output":"0\n"},{"input":"2 3\n1 6 4\n3 2 5\n","output":"1 3\n"},{"input":"1 6\n1 6 5 4 3 2\n","output":"2\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"EPiratSerezha"}}}

use std::cmp::min;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read();
    let m = input.read();
    let mut a = vec![vec![-100; m]; n];
    let mut count_have = 0;
    let mut has = vec![false; n];
    for row in 0..n {
        for col in 0..m {
            a[row][col] = input.read();
            if row > 0 && a[row - 1][col] == a[row][col] - 1 {
                has[a[row - 1][col]] = true;
                count_have += 1;
            }
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
