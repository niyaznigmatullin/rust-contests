//{"name":"E. Уничтожение стены","group":"Codeforces - Codeforces Round #786 (Div. 3)","url":"https://codeforces.com/contest/1674/problem/E","interactive":false,"timeLimit":2000,"tests":[{"input":"5\n20 10 30 10 20\n","output":"10\n"},{"input":"3\n1 8 1\n","output":"1\n"},{"input":"6\n7 6 6 8 5 8\n","output":"4\n"},{"input":"6\n14 3 8 10 15 4\n","output":"4\n"},{"input":"4\n1 100 100 1\n","output":"2\n"},{"input":"3\n40 10 10\n","output":"7\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"EUnichtozhenieSteni"}}}

use std::cmp::{max, min};
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read();
    let a = input.read_vec(n);
    out_line!(min(solve_over_1(&a), min(solve_2_min(&a), solve_nei(&a))));
}

fn solve_over_1(a: &Vec<u32>) -> u32 {
    if a.len() < 3 {
        return u32::MAX;
    }
    let mut ans = u32::MAX;
    for i in 0..a.len() - 2 {
        let x = max(a[i], a[i + 2]);
        let y = min(a[i], a[i + 2]);
        let cur = y + (x - y + 1) / 2;
        ans = min(ans, cur);
    }
    ans
}

fn solve_nei(a: &Vec<u32>) -> u32 {
    let mut ans = u32::MAX;
    for i in 0..a.len() - 1 {
        let mut x = max(a[i], a[i + 1]);
        let y = min(a[i], a[i + 1]);
        if 2 * y <= x {
            ans = min(ans, (x + 1) / 2);
        } else {
            let mut cur = x - y;
            x -= 2 * cur;
            cur += x / 3 * 2;
            x %= 3;
            if x != 0 {
                if x == 1 {
                    cur += 1;
                } else {
                    cur += 2;
                }
            }
            ans = min(ans, cur);
        }
    }
    ans
}

fn solve_2_min(a: &Vec<u32>) -> u32 {
    let mut m1 = u32::MAX;
    let mut m2 = u32::MAX;
    for &x in a {
        if x < m1 {
            m2 = m1;
            m1 = x;
        } else if x < m2 {
            m2 = x;
        }
    }
    (m1 + 1) / 2 + (m2 + 1) / 2
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
