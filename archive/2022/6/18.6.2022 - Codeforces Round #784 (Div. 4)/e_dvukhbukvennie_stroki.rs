//{"name":"E. Двухбуквенные строки","group":"Codeforces - Codeforces Round #784 (Div. 4)","url":"https://codeforces.com/contest/1669/problem/E","interactive":false,"timeLimit":2000,"tests":[{"input":"4\n6\nab\ncb\ndb\naa\ncc\nef\n7\naa\nbb\ncc\nac\nca\nbb\naa\n4\nkk\nkk\nab\nab\n5\njf\njf\njk\njk\njk\n","output":"5\n6\n0\n6\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"EDvukhbukvennieStroki"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read();
    let mut count = vec![vec![0; 11]; 11];
    let mut ans : u64 = 0;
    for _ in 0..n {
        let s = input.read::<String>().into_bytes();
        let first = (s[0] - b'a') as usize;
        let second = (s[1] - b'a') as usize;
        let mut cur_ans = 0;
        for i in 0..11 {
            if i != first {
                cur_ans += count[i][second];
            }
            if i != second {
                cur_ans += count[first][i];
            }
        }
        ans += cur_ans;
        count[first][second] += 1;
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
