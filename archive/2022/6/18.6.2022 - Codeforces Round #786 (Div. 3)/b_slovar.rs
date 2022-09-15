//{"name":"B. Словарь","group":"Codeforces - Codeforces Round #786 (Div. 3)","url":"https://codeforces.com/contest/1674/problem/B","interactive":false,"timeLimit":2000,"tests":[{"input":"7\nab\nac\naz\nba\nbc\nzx\nzy\n","output":"1\n2\n25\n26\n27\n649\n650\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BSlovar"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out, out_line};

fn solve(input: &mut Input, _test_case: usize, d: &Vec<Vec<i32>>) {
    let s = input.read::<String>().into_bytes();
    out_line!(d[(s[0] - b'a') as usize][(s[1] - b'a') as usize]);
}

pub(crate) fn run(mut input: Input) -> bool {
    let mut d = vec![vec![0; 26]; 26];
    let mut cnt = 0;
    for i in 0..26 {
        for j in 0..26 {
            if i != j {
                cnt += 1;
                d[i][j] = cnt;
            }
        }
    }
    let t = input.read();
    for i in 0usize..t {
        solve(&mut input, i + 1, &d);
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
