//{"name":"H. Максимальный AND","group":"Codeforces - Codeforces Round #784 (Div. 4)","url":"https://codeforces.com/contest/1669/problem/H","interactive":false,"timeLimit":2000,"tests":[{"input":"4\n3 2\n2 1 1\n7 0\n4 6 6 28 6 6 12\n1 30\n0\n4 4\n3 1 3 1\n","output":"2\n4\n2147483646\n1073741825\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"HMaksimalniiAND"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let (n, mut k) = input.read();
    let a = input.read_vec::<u32>(n);
    let mut count_bits = vec![0; 31];
    for &x in &a {
        for bit in 0..31 {
            if ((x >> bit) & 1) == 1 {
                count_bits[bit] += 1;
            }
        }
    }
    let mut ans = 0u32;
    for bit in (0..31).rev() {
        if n - count_bits[bit] <= k {
            k -= n - count_bits[bit];
            ans |= 1u32 << bit;
        }
    }
    out_line!(ans);
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
