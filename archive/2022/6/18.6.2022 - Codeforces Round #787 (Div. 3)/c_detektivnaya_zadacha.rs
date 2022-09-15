//{"name":"C. Детективная задача","group":"Codeforces - Codeforces Round #787 (Div. 3)","url":"https://codeforces.com/contest/1675/problem/C?locale=ru","interactive":false,"timeLimit":2000,"tests":[{"input":"8\n0\n1\n1110000\n?????\n1?1??0?0\n0?0???\n??11\n??0??\n","output":"1\n1\n2\n5\n4\n1\n1\n3\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CDetektivnayaZadacha"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let a = input.read::<String>().into_bytes();
    let mut has_1: Vec<_> = a.iter().map(|x| if *x == b'1' {
        true
    } else {
        false
    }).collect();
    let mut has_0: Vec<_> = a.iter().map(|x| if *x == b'0' {
        true
    } else {
        false
    }).collect();
    let n = a.len();
    for i in 1..n {
        has_0[i] |= has_0[i - 1];
    }
    for i in (0..n - 1).rev() {
        has_1[i] |= has_1[i + 1];
    }
    let mut ans = 0;
    for i in 0..n {
        if (i == 0 || !has_0[i - 1]) && (i + 1 == n || !has_1[i + 1]) {
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
