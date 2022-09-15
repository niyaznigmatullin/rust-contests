//{"name":"F. Пазл","group":"Codeforces - Codeforces Round #802 (Div. 2)","url":"https://codeforces.com/contest/1700/problem/F","interactive":false,"timeLimit":1000,"tests":[{"input":"5\n0 1 0 1 0\n1 1 0 0 1\n1 0 1 0 1\n0 0 1 1 0\n","output":"5\n"},{"input":"3\n1 0 0\n0 0 0\n0 0 0\n0 0 0\n","output":"-1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"FPazl"}}}

use std::cmp::{max, min};
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let n_do_not_use = input.read();
    let start = read_field(input, n_do_not_use);
    let finish = read_field(input, n_do_not_use);
    if start.len() != finish.len() {
        out_line!(-1);
        return;
    }
    let mut i = 0;
    let mut f_strict = 0;
    let mut f_skip = i32::MAX;
    while i < finish.len() {
        if i + 1 < finish.len() && finish[i].1 == finish[i + 1].1 {
            let mut nf_strict = i32::MAX;
            let mut nf_skip = i32::MAX;
            for first in [i - 1, i] {
                let prev_value = if first == i { f_strict } else { f_skip };
                if prev_value == i32::MAX {
                    continue;
                }
                for second in [i + 1, i + 2] {
                    if second >= finish.len() {
                        continue;
                    }
                    for to_first in [i, i + 1] {
                        let to_second = 2 * i + 1 - to_first;
                        let value = prev_value + dist(finish[first], start[to_first]) + dist(finish[second], start[to_second]);
                        if second == i + 2 {
                            nf_skip = min(nf_skip, value);
                        } else {
                            nf_strict = min(nf_strict, value);
                        }
                    }
                }
            }
            f_strict = nf_strict;
            f_skip = nf_skip;
            i += 2;
        } else {
            let mut nf_strict = f_strict + dist(finish[i], start[i]);
            if f_skip != i32::MAX {
                nf_strict = min(nf_strict, f_skip + dist(finish[i], start[i - 1]));
            }
            let nf_skip = if i + 1 < finish.len() {
                dist(finish[i], start[i + 1]) + f_strict
            } else {
                i32::MAX
            };
            f_strict = nf_strict;
            f_skip = nf_skip;
            i += 1;
        }
    }
    out_line!(f_strict);
}

fn dist(p: (usize, usize), q: (usize, usize)) -> i32 {
    let d_row = p.0 as i32 - q.0 as i32;
    let d_col = p.1 as i32 - q.1 as i32;
    (d_row.abs() + d_col.abs()) as i32
}

fn read_field(input: &mut Input, n: usize) -> Vec<(usize, usize)> {
    let mut field = Vec::new();
    field.push(input.read_vec::<u8>(n));
    field.push(input.read_vec(n));
    let mut res = Vec::new();
    for col in 0..n {
        for row in 0..2 {
            if field[row][col] == 1 {
                res.push((row, col));
            }
        }
    }
    res
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
