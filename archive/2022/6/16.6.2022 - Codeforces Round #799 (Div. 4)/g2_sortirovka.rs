//{"name":"G. 2^Сортировка","group":"Codeforces - Codeforces Round #799 (Div. 4)","url":"https://codeforces.com/contest/1692/problem/G","interactive":false,"timeLimit":1000,"tests":[{"input":"6\n4 2\n20 22 19 84\n5 1\n9 5 3 2 1\n5 2\n9 5 3 2 1\n7 2\n22 12 16 4 3 22 12\n7 3\n22 12 16 4 3 22 12\n9 3\n3 9 12 3 9 12 3 9 12\n","output":"2\n3\n2\n3\n1\n0\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"G2Sortirovka"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read();
    let k = input.read();
    let a : Vec<_> = (0..n).map(|_| input.read::<u32>()).collect();
    let mut s = vec![0; n - 1];
    for i in 0..n - 1 {
        s[i] = if a[i] >= 2 * a[i + 1] {
            0
        } else {
            1
        }
    }
    for i in 1..n - 1 {
        s[i] += s[i - 1];
    }
    let mut ans = 0;
    for i in k - 1..n - 1 {
        let mut got = s[i];
        if i >= k {
            got -= s[i - k];
        }
        if got == k {
            ans += 1;
        }
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
