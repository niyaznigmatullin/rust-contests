//{"name":"B. Расшифруй строку","group":"Codeforces - Codeforces Round #820 (Div. 3)","url":"https://codeforces.com/contest/1729/problem/B","interactive":false,"timeLimit":1000,"tests":[{"input":"9\n6\n315045\n4\n1100\n7\n1213121\n6\n120120\n18\n315045615018035190\n7\n1111110\n7\n1111100\n5\n11111\n4\n2606\n","output":"code\naj\nabacaba\nll\ncodeforces\naaaak\naaaaj\naaaaa\nzf\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BRasshifruiStroku"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let _ = input.read::<i32>();
    let s = input.read::<String>().into_bytes();
    let mut i = s.len();
    let mut ans = Vec::new();
    while i > 0 {
        let prev = s[i - 1];
        i -= 1;
        if prev == b'0' {
            assert!(i >= 2);
            let num = String::from_utf8_lossy(&s[i - 2..i]).parse::<u8>().unwrap();
            ans.push(b'a' + num - 1);
            i -= 2;
        } else {
            ans.push(b'a' + prev - b'0' - 1);
        }
    }
    ans.reverse();
    let ans = String::from_utf8_lossy(&ans).to_string();
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
