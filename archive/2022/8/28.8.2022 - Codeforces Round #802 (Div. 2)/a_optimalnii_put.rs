//{"name":"A. Оптимальный путь","group":"Codeforces - Codeforces Round #802 (Div. 2)","url":"https://codeforces.com/contest/1700/problem/A","interactive":false,"timeLimit":2000,"tests":[{"input":"7\n1 1\n2 3\n3 2\n7 1\n1 10\n5 5\n10000 10000\n","output":"1\n12\n13\n28\n55\n85\n500099995000\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"AOptimalniiPut"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let n : i64 = input.read();
    let m : i64 = input.read();
    let mut ans = m * (m - 1) / 2 + 1 * m;
    if n > 1 {
        ans += (n - 1) * (n - 2) / 2 * m + 2 * m * (n - 1);
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
