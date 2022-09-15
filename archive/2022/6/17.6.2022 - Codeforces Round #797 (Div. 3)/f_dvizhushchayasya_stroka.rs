//{"name":"F. Движущаяся строка","group":"Codeforces - Codeforces Round #797 (Div. 3)","url":"https://codeforces.com/contest/1690/problem/F","interactive":false,"timeLimit":3000,"tests":[{"input":"3\n5\nababa\n3 4 5 2 1\n5\nababa\n2 1 4 5 3\n10\ncodeforces\n8 6 1 7 5 2 9 3 10 4\n","output":"1\n6\n12\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"FDvizhushchayasyaStroka"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out, out_line};
use algo_lib::math::euclid::lcm;
use algo_lib::strings::prefix_function::prefix_function;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read();
    let s = input.read::<String>().into_bytes();
    let mut p = input.read_vec::<usize>(n);
    for x in &mut p {
        *x -= 1;
    }
    let mut used = vec![false; n];
    let mut ans = 1;
    for i in 0..n {
        if used[i] {
            continue;
        }
        let mut cur = i;
        let mut str = Vec::new();
        while !used[cur] {
            used[cur] = true;
            str.push(s[cur]);
            cur = p[cur];
        }
        let period = calc_period(&str);
        ans = lcm(ans, period as u64);
    }
    out_line!(ans);
}

fn calc_period(str: &Vec<u8>) -> usize {
    let p = prefix_function(&str);
    let n = str.len();
    let mut i = p[n - 1];
    while n % (n - i) != 0 {
        i = p[i - 1];
    }
    n - i
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
