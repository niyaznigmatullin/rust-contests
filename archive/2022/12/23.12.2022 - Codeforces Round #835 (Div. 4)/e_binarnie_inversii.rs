//{"name":"E. Бинарные инверсии","group":"Codeforces - Codeforces Round #835 (Div. 4)","url":"https://codeforces.com/contest/1760/problem/E","interactive":false,"timeLimit":2000,"tests":[{"input":"5\n4\n1 0 1 0\n6\n0 1 0 0 1 0\n2\n0 0\n8\n1 0 1 1 0 0 0 1\n3\n1 1 1\n","output":"3\n7\n1\n13\n2\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"EBinarnieInversii"}}}

use std::cmp::max;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read();
    let a = input.read_vec::<i32>(n);
    let mut count_zeros = a.iter().filter(|&&x| x == 0).count();
    let mut count_ones = 0;
    let mut all_inversions = 0;
    let mut best_delta = 0;
    for x in a {
        if x == 1 {
            if count_ones > count_zeros {
                best_delta = max(best_delta, count_ones - count_zeros);
            }
            all_inversions += count_zeros;
            count_ones += 1;
        } else {
            count_zeros -= 1;
            if count_zeros > count_ones {
                best_delta = max(best_delta, count_zeros - count_ones);
            }
        }
    }
    out_line!(all_inversions + best_delta);
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
