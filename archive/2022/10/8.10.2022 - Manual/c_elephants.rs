//{"name":"c_elephants","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"c_elephants"}}}

use algo_lib::datastructures::util::subtract_one;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out, out_line};
use std::cmp::min;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read();
    let masses = input.read_vec(n);
    let old = subtract_one(input.read_vec::<usize>(n));
    let new = subtract_one(input.read_vec::<usize>(n));
    let mut t = old
        .iter()
        .copied()
        .zip(new.iter().copied())
        .collect::<Vec<_>>();
    t.sort();
    let t = t.into_iter().map(|x| x.1).collect::<Vec<_>>();
    let best_overall = *masses.iter().min().unwrap();
    let mut used = vec![false; n];
    let mut ans = 0i64;
    for mut i in 0..n {
        if used[i] {
            continue;
        }
        let mut sum = 0;
        let mut best = i64::MAX;
        let mut count = 0;
        while !used[i] {
            used[i] = true;
            sum += masses[i];
            best = min(best, masses[i]);
            count += 1;
            i = t[i];
        }
        ans += (sum - best) + min(best * (count - 1), (best + best_overall + best + best_overall * count));
    }
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
