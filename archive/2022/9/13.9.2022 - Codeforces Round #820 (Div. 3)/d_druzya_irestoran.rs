//{"name":"D. Друзья и ресторан","group":"Codeforces - Codeforces Round #820 (Div. 3)","url":"https://codeforces.com/contest/1729/problem/D","interactive":false,"timeLimit":2000,"tests":[{"input":"6\n6\n8 3 9 2 4 5\n5 3 1 4 5 10\n4\n1 2 3 4\n1 1 2 2\n3\n2 3 7\n1 3 10\n6\n2 3 6 9 5 7\n3 2 7 10 6 10\n6\n5 4 2 1 8 100\n1 1 1 1 1 200\n6\n1 4 1 2 4 2\n1 3 3 2 3 4\n","output":"2\n0\n1\n3\n1\n3\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DDruzyaIRestoran"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read();
    let spend = input.read_vec::<i64>(n);
    let budget = input.read_vec::<i64>(n);
    let mut diffs = Vec::new();
    for (spend, budget) in spend.into_iter().zip(budget) {
        diffs.push(budget - spend);
    }
    diffs.sort();
    let mut i = 0;
    let mut ans = 0;
    for j in (0..diffs.len()).rev() {
        while i < j && diffs[i] + diffs[j] < 0 {
            i += 1;
        }
        if i >= j {
            break;
        }
        ans += 1;
        i += 1;
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
