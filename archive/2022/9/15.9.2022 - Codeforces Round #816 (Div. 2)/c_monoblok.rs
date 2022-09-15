//{"name":"C. Моноблок","group":"Codeforces - Codeforces Round #816 (Div. 2)","url":"https://codeforces.com/contest/1715/problem/C?locale=ru","interactive":false,"timeLimit":1000,"tests":[{"input":"5 5\n1 2 3 4 5\n3 2\n4 2\n3 1\n2 1\n2 2\n","output":"29\n23\n35\n25\n35\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CMonoblok"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read();
    let m = input.read();
    let mut a = input.read_vec::<i32>(n);
    let mut coef = vec![0; n];
    for i in 0..n {
        coef[i] = i as i64 * (n - i) as i64;
    }
    let mut ans = n as i64 * (n + 1) as i64 / 2;
    for i in 1..n {
        if a[i] != a[i - 1] {
            ans += coef[i];
        }
    }
    for _ in 0..m {
        let pos = input.read::<usize>();
        let pos = pos - 1;
        let value = input.read();
        if pos > 0 && a[pos] != a[pos - 1] {
            ans -= coef[pos];
        }
        if pos + 1 < n && a[pos] != a[pos + 1] {
            ans -= coef[pos + 1];
        }
        a[pos] = value;
        if pos > 0 && a[pos] != a[pos - 1] {
            ans += coef[pos];
        }
        if pos + 1 < n && a[pos] != a[pos + 1] {
            ans += coef[pos + 1];
        }
        out_line!(ans);
    }
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
