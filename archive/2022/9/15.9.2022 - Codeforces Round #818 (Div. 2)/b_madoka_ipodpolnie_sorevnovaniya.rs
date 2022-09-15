//{"name":"B. Мадока и подпольные соревнования","group":"Codeforces - Codeforces Round #818 (Div. 2)","url":"https://codeforces.com/contest/1717/problem/B","interactive":false,"timeLimit":1000,"tests":[{"input":"3\n3 3 3 2\n2 1 1 2\n6 3 4 2\n","output":"X..\n..X\n.X.\nXX\nXX\n.X..X.\nX..X..\n..X..X\n.X..X.\nX..X..\n..X..X\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BMadokaIPodpolnieSorevnovaniya"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let (n, k, r, c): (_, usize, usize, usize) = input.read();
    let mut ans = vec![vec![0u8; n]; n];
    let r = r - 1;
    let c = c - 1;
    for x in 0..n {
        for y in 0..n {
            ans[x][y] = if ((x + y) + k - (r + c) % k) % k == 0 {
                b'X'
            } else {
                b'.'
            }
        }
    }
    for row in ans {
        out_line!(String::from_utf8_lossy(&row).to_string());
    }
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
