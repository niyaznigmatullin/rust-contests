//{"name":"C. Помогаем природе","group":"Codeforces - Codeforces Round #802 (Div. 2)","url":"https://codeforces.com/contest/1700/problem/C","interactive":false,"timeLimit":2000,"tests":[{"input":"4\n3\n-2 -2 -2\n3\n10 4 7\n4\n4 -4 4 -4\n5\n1 -2 3 -4 5\n","output":"2\n13\n36\n33\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CPomogaemPrirode"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read();
    let a = input.read_vec::<i64>(n);
    let mut sub = 0i64;
    let mut ans = 0i64;
    for i in 1..n {
        if a[i - 1] > a[i] {
            sub += a[i - 1] - a[i];
            ans += a[i - 1] - a[i];
        } else {
            ans += a[i] - a[i - 1];
        }
    }
    ans += (a[0] - sub).abs();
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
