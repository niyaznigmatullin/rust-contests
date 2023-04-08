//{"name":"C. Разные разности","group":"Codeforces - Codeforces Round #839 (Div. 3)","url":"https://codeforces.com/contest/1772/problem/C","interactive":false,"timeLimit":2000,"tests":[{"input":"7\n5 9\n4 12\n3 3\n3 4\n4 4\n4 6\n8 11\n","output":"1 3 4 7 8\n2 4 7 12\n1 2 3\n1 3 4\n1 2 3 4\n2 4 5 6\n1 2 3 5 6 7 8 11\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CRaznieRaznosti"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let k = input.read();
    let n = input.read();
    for i in (1..k).rev() {
        if 1 + i * (i + 1) / 2 + (k - i - 1) > n {
            continue;
        }
        let mut ans = Vec::new();
        let mut cur = 1;
        ans.push(cur);
        for j in 1..=i {
            cur += j;
            ans.push(cur);
        }
        for j in (i + 1)..k {
            cur += 1;
            ans.push(cur);
        }
        out_line!(ans);
        return;
    }
    assert!(false);
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
