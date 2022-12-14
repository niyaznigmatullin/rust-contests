//{"name":"A. Дивизон?","group":"Codeforces - Codeforces Round #784 (Div. 4)","url":"https://codeforces.com/contest/1669/problem/A","interactive":false,"timeLimit":1000,"tests":[{"input":"7\n-789\n1299\n1300\n1399\n1400\n1679\n2300\n","output":"Division 4\nDivision 4\nDivision 4\nDivision 4\nDivision 3\nDivision 2\nDivision 1\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"ADivizon"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let rating = input.read::<i32>();
    let division = match rating {
        x if x <= 1399 => 4,
        x if x <= 1599 => 3,
        x if x <= 1899 => 2,
        _ => 1,
    };
    out_line!(format!("Division {division}"));
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
