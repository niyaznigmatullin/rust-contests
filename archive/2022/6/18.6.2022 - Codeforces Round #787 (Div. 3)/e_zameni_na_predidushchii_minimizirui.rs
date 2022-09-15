//{"name":"E. Замени на предыдущий, минимизируй","group":"Codeforces - Codeforces Round #787 (Div. 3)","url":"https://codeforces.com/contest/1675/problem/E","interactive":false,"timeLimit":2000,"tests":[{"input":"4\n3 2\ncba\n4 5\nfgde\n7 5\ngndcafb\n4 19\nekyv\n","output":"aaa\nagaa\nbnbbabb\naapp\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"EZameniNaPredidushchiiMinimizirui"}}}

use std::cmp::min;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let _ = input.read::<u32>();
    let mut k = min(input.read::<u32>(), 26) as u8;
    let mut s : Vec<_> = input.read::<String>()
        .into_bytes()
        .iter()
        .map(|&c| c - b'a')
        .collect();
    let mut from_to_zero = 0;
    let mut second : Option<(u8, u8)> = None;
    for c in &mut s {
        if *c <= from_to_zero {
            *c = 0;
        } else if *c - from_to_zero <= k {
            k -= *c - from_to_zero;
            from_to_zero = *c;
            *c = 0;
        } else {
            if let &Some((from, to)) = &second {
                if *c >= from && *c <= to {
                    *c = from;
                }
            } else {
                second = Some((*c - k, *c));
                *c -= k;
                k = 0;
            }
        }
    }
    for c in &mut s {
        *c += b'a';
    }
    out_line!(String::from_utf8(s).unwrap());
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
