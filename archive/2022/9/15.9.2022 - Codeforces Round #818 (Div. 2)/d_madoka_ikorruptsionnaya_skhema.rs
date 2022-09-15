//{"name":"D. Мадока и коррупционная схема","group":"Codeforces - Codeforces Round #818 (Div. 2)","url":"https://codeforces.com/contest/1717/problem/D","interactive":false,"timeLimit":1000,"tests":[{"input":"1 1\n","output":"2\n"},{"input":"2 1\n","output":"3\n"},{"input":"3 2\n","output":"7\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DMadokaIKorruptsionnayaSkhema"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out, out_line};
use std::cmp::min;
use algo_lib::math::modular::factorial::Factorial;
use algo_lib::math::modular::primitive::ModularType;

type Mod = ModularType<1000000007>;

fn solve(input: &mut Input, _test_case: usize) {
    let (n, k) = input.read();
    let mut ans = Mod::from(0);
    let factorials = Factorial::new(n);
    for i in 0..=min(k, n) {
        ans += factorials.combinations(n, i);
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
