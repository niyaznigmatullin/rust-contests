//{"name":"C: Balance Scale","group":"Meta Coding Competitions - Meta Hacker Cup 2022 Round 2","url":"https://www.facebook.com/codingcompetitions/hacker-cup/2022/round-2/problems/C","interactive":false,"timeLimit":360000,"tests":[{"input":"6\n5 1\n1 3000\n1 2000\n1 1000\n1 2000\n1 1000\n5 2\n1 3000\n1 2000\n1 1000\n1 2000\n1 1000\n2 10\n10 1\n10 2\n5 2\n2 50\n1 40\n1 50\n1 60\n3 50\n4 2993\n3000 999999999\n2995 1000000000\n1552 888888888\n1336 999999999\n3 1\n1 10\n2 9\n1 11\n","output":"Case #1: 800000006\nCase #2: 200000002\nCase #3: 0\nCase #4: 208333335\nCase #5: 590307096\nCase #6: 333333336\n"}],"testType":"multiNumber","input":{"type":"regex","fileName":null,"pattern":"balance_scale_.*input[.]txt"},"output":{"type":"file","fileName":"balance_scale_output.txt","pattern":null},"languages":{"java":{"taskClass":"CBalanceScale"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::math::modular::factorial::Factorial;
use algo_lib::math::modular::primitive::{Modular, ModularType};
use algo_lib::{out, out_line};
use std::cmp::min;

type Mod = ModularType<1000000007>;

fn solve(input: &mut Input, _test_case: usize) {
    out!(format!("Case #{}: ", _test_case));
    let (n, k): (usize, usize) = input.read();
    let k = k + 1;
    let a = input.read_vec::<(usize, i32)>(n);
    let count_all = a.iter().map(|(count, _)| *count).sum::<usize>();
    let factorials = Factorial::new(count_all + 1);
    let mut ways = Mod::from(0);
    let count_need = a[0].0;
    let count_same_weight = a
        .iter()
        .filter(|(_, w)| *w == a[0].1)
        .map(|(count, _)| *count)
        .sum::<usize>();
    let count_smaller_weight = a
        .iter()
        .filter(|(_, w)| *w < a[0].1)
        .map(|(count, _)| *count)
        .sum::<usize>();
    for k1 in 1..=min(k, count_same_weight) {
        ways += Mod::from(count_need)
            * factorials.combinations(count_same_weight - 1, k1 - 1)
            * factorials.factorial(k1 - 1)
            * factorials.combinations(count_smaller_weight, k - k1)
            * factorials.factorial(k - k1)
            * factorials.combinations(k, k1);
    }
    ways *= factorials.inverse_combinations(count_all, k)
        * factorials.inverse_factorial(k);
    out_line!(ways);
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
