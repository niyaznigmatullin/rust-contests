//{"name":"b_conference_room","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"b_conference_room"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::strings::duval::lex_min_cyclic_shift;
use algo_lib::{out, out_line};
use std::collections::hash_map::RandomState;
use std::collections::HashSet;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read();
    let a = (0..4).map(|_| input.read_vec::<i32>(n)).collect::<Vec<_>>();
    let sum = a
        .iter()
        .map(|x| x.into_iter())
        .flatten()
        .map(|x| *x as i64)
        .sum::<i64>();
    out!(format!("Case {}: ", _test_case));
    if sum % n as i64 != 0 {
        out_line!("No");
        return;
    }
    let sum = (sum / n as i64) as i32;
    let sums1 = get_sums(&a[0], &a[1]);
    let table =
        HashSet::<_, RandomState>::from_iter(sums1.into_iter().map(|x| lex_min_cyclic_shift(x)));
    let sums2 = get_sums(&a[2], &a[3]);
    for mut x in sums2 {
        for i in &mut x {
            *i = sum - *i;
        }
        x = lex_min_cyclic_shift(x);
        if table.contains(&x) {
            out_line!("Yes");
            return;
        }
    }
    out_line!("No");
}

fn get_sums(a: &[i32], b: &[i32]) -> Vec<Vec<i32>> {
    let n = b.len();
    (0..n)
        .map(|shift| {
            let mut res = vec![0; n];
            for i in 0..n {
                res[i] = a[i] + b[(i + shift) % n];
            }
            res
        })
        .collect()
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
