//{"name":"A: Second Hands","group":"Meta Coding Competitions - Meta Hacker Cup 2022 Qualification Round","url":"https://www.facebook.com/codingcompetitions/hacker-cup/2022/qualification-round/problems/A","interactive":false,"timeLimit":360000,"tests":[{"input":"5\n3 2\n1 2 2\n5 3\n1 2 3 3 1\n5 2\n1 2 3 4 5\n5 5\n1 1 2 2 1\n1 1\n1\n","output":"Case #1: YES\nCase #2: YES\nCase #3: NO\nCase #4: NO\nCase #5: YES\n"}],"testType":"multiNumber","input":{"type":"regex","fileName":null,"pattern":"second_hands_.*input[.]txt"},"output":{"type":"file","fileName":"second_hands_output.txt","pattern":null},"languages":{"java":{"taskClass":"ASecondHands"}}}

use std::collections::HashMap;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out, out_line};

fn solve(input: &mut Input, test_case: usize) {
    out!(format!("Case #{}: ", test_case));
    let n = input.read();
    let k: usize = input.read();
    let a = input.read_vec::<i32>(n);
    if 2 * k < n {
        out_line!("NO");
        return;
    }
    let mut table = HashMap::new();
    for x in a {
        *table.entry(x).or_insert(0) += 1;
    }
    if table.values().any(|&count| count > 2) {
        out_line!("NO");
        return;
    }
    out_line!("YES");
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
