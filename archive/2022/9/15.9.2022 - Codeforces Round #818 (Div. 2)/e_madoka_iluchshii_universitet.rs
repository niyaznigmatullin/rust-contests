//{"name":"E. Мадока и лучший университет","group":"Codeforces - Codeforces Round #818 (Div. 2)","url":"https://codeforces.com/contest/1717/problem/E","interactive":false,"timeLimit":1000,"tests":[{"input":"3\n","output":"1\n"},{"input":"5\n","output":"11\n"},{"input":"69228\n","output":"778304278\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"EMadokaILuchshiiUniversitet"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::math::euclid::gcd;
use algo_lib::math::sieve::{euler_function_up_to, get_all_divisors_for_all_up_to};
use algo_lib::{out, out_line};
use algo_lib::math::modular::primitive::ModularType;

type Mod = ModularType<1000000007>;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read();
    let phi = euler_function_up_to(n);
    let divisors = get_all_divisors_for_all_up_to(n);
    let mut ans = Mod::from(0);
    for c in 1..n - 1 {
        for &g in &divisors[n - c] {
            if g == n - c {
                continue;
            }
            let x = gcd(g as u64, c as u64) as usize;
            ans += Mod::from(c / x) * g.into() * phi[(n - c) / g].into();
        }
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
