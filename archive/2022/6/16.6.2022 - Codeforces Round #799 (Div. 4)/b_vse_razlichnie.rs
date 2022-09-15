//{"name":"B. Все различные","group":"Codeforces - Codeforces Round #799 (Div. 4)","url":"https://codeforces.com/contest/1692/problem/B","interactive":false,"timeLimit":1000,"tests":[{"input":"4\n6\n2 2 2 3 3 3\n5\n9 1 9 9 1\n4\n15 16 16 15\n4\n10 100 1000 10000\n","output":"2\n1\n2\n4\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BVseRazlichnie"}}}

use std::iter::repeat;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read();
    let mut a: Vec<_> = (0..n).map(|_| input.read::<i32>()).collect();
    a.sort();
    let mut uniq = 1;
    for i in 1..n {
        if a[i] != a[i - 1] {
            uniq += 1;
        }
    }
    out_line!(n - (n - uniq + 1) / 2 * 2);
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
