//{"name":"C. Мадока и формальное условие","group":"Codeforces - Codeforces Round #818 (Div. 2)","url":"https://codeforces.com/contest/1717/problem/C","interactive":false,"timeLimit":1000,"tests":[{"input":"5\n3\n1 2 5\n1 2 5\n2\n2 2\n1 3\n4\n3 4 1 2\n6 4 2 5\n3\n2 4 1\n4 5 3\n5\n1 2 3 4 5\n6 5 6 7 6\n","output":"YES\nNO\nNO\nNO\nYES\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CMadokaIFormalnoeUslovie"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read();
    let a = input.read_vec::<i32>(n);
    let b = input.read_vec::<i32>(n);
    for i in 0..n {
        let j = (i + 1) % n;
        if a[i] < b[i] && b[i] > b[j] + 1 || a[i] > b[i] {
            out_line!("NO");
            return;
        }
    }
    out_line!("YES");
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
