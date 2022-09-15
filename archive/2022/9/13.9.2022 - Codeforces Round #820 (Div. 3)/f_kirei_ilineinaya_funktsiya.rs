//{"name":"F. Кирей и линейная функция","group":"Codeforces - Codeforces Round #820 (Div. 3)","url":"https://codeforces.com/contest/1729/problem/F","interactive":false,"timeLimit":3000,"tests":[{"input":"5\n1003004\n4 1\n1 2 1\n179572007\n4 2\n2 7 3\n2 7 4\n111\n2 1\n2 2 6\n0000\n1 2\n1 4 0\n1 4 1\n484\n1 5\n2 2 0\n2 3 7\n1 2 5\n3 3 8\n2 2 6\n","output":"2 4\n1 5\n1 2\n-1 -1\n1 2\n-1 -1\n1 3\n1 3\n-1 -1\n-1 -1\n-1 -1\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"FKireiILineinayaFunktsiya"}}}

use algo_lib::datastructures::prefix_sum::StaticRSQ;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out, out_line};
use std::cmp::min;

fn solve(input: &mut Input, _test_case: usize) {
    let s = input
        .read::<String>()
        .into_bytes()
        .into_iter()
        .map(|x| (x - b'0') as usize)
        .collect::<Vec<_>>();
    let s = StaticRSQ::new(s);
    let (w, m): (usize, _) = input.read();
    let mut substrings = vec![Vec::new(); 9];
    for i in 0..=(s.len() - w) {
        substrings[s.get_sum(i..i + w) % 9].push(i);
    }
    for _ in 0..m {
        let (mut left, right, k): (usize, usize, usize) = input.read();
        left -= 1;
        let sub_mod = s.get_sum(left..right);
        let ans = (0..9)
            .map(|a| {
                let b = (k + 9 - sub_mod * a % 9) % 9;
                if a == b {
                    if substrings[a].len() > 1 {
                        Some((substrings[a][0], substrings[a][1]))
                    } else {
                        None
                    }
                } else {
                    if substrings[a].is_empty() || substrings[b].is_empty() {
                        None
                    } else {
                        Some((substrings[a][0], substrings[b][0]))
                    }
                }
            })
            .flatten()
            .fold(None, |acc, x| {
                Some(match acc {
                    None => x,
                    Some(y) => min(x, y),
                })
            });
        if let Some((x, y)) = ans {
            out_line!(x + 1, y + 1);
        } else {
            out_line!("-1 -1");
        }
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
