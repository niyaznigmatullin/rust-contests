//{"name":"D. Долины с трудностями","group":"Codeforces - Codeforces Round #835 (Div. 4)","url":"https://codeforces.com/contest/1760/problem/D","interactive":false,"timeLimit":2000,"tests":[{"input":"6\n7\n3 2 2 1 2 2 3\n11\n1 1 1 2 3 3 4 5 6 6 6\n7\n1 2 3 4 3 2 1\n7\n9 7 4 6 9 9 10\n1\n1000000000\n8\n9 4 4 5 9 4 9 10\n","output":"YES\nYES\nNO\nYES\nYES\nNO\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DDoliniSTrudnostyami"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read();
    let a = input.read_vec::<i32>(n);
    let a = {
        let mut b = Vec::new();
        for x in a {
            match b.last() {
                Some(&y) if y == x => (),
                _ => b.push(x),
            }
        }
        b
    };
    let mut count = 0;
    for i in 0..a.len() {
        if (i == 0 || a[i - 1] > a[i]) && (i + 1 == a.len() || a[i + 1] > a[i]) {
            count += 1;
            if count >= 2 {
                break;
            }
        }
    }
    out_line!(if count == 1 { "YES" } else { "NO" });
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
