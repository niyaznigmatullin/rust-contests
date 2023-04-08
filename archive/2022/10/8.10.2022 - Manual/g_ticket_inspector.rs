//{"name":"g_ticket_inspector","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"g_ticket_inspector"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out, out_line};
use std::cmp::{max, min};

#[derive(Clone)]
struct Dp {
    value: i32,
    sequence: Vec<usize>,
}

fn solve(input: &mut Input, _test_case: usize) {
    let (n, k) = input.read();
    let mut a = vec![vec![0; n]; n];
    for start in 0..n {
        for finish in start + 1..n {
            a[start][finish] = input.read();
        }
    }
    let mut f = vec![Vec::<Dp>::new(); n - 1];
    for last in 0..n - 1 {
        let mut sum = 0;
        let mut cur_f = vec![
            Dp {
                value: i32::MIN,
                sequence: Vec::new()
            };
            k
        ];
        for pre_last in (0..=last).rev() {
            if pre_last < last {
                for count in 1..k {
                    let value = f[pre_last][count - 1].value + sum;
                    if cur_f[count].value < value {
                        cur_f[count] = Dp {
                            value,
                            sequence: {
                                let mut z = f[pre_last][count - 1].sequence.clone();
                                z.push(last);
                                z
                            },
                        };
                    }
                }
            }
            let row = &a[pre_last];
            for finish in last + 1..n {
                sum += row[finish];
            }
        }
        cur_f[0] = Dp {
            value: sum,
            sequence: vec![last],
        };
        f[last] = cur_f;
    }
    let ans = f
        .iter()
        .map(|x| x.into_iter())
        .flatten()
        .max_by_key(|x| x.value)
        .unwrap();
    let ans = ans.sequence.iter().map(|x| x + 1).collect::<Vec<_>>();
    out_line!(ans);
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
