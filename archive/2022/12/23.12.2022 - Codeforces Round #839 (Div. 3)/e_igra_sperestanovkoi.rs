//{"name":"E. Игра с перестановкой","group":"Codeforces - Codeforces Round #839 (Div. 3)","url":"https://codeforces.com/contest/1772/problem/E","interactive":false,"timeLimit":4000,"tests":[{"input":"4\n4\n1 2 4 3\n3\n2 3 1\n5\n3 4 5 2 1\n6\n1 5 6 3 2 4\n","output":"First\nTie\nSecond\nTie\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"EIgraSPerestanovkoi"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read();
    let a = input.read_vec::<usize>(n);
    let mut first = 0;
    let mut second = 0;
    let mut both = 0;
    for i in 0..n {
        let first_fixed = a[i] == i + 1;
        let second_fixed = a[i] == n - i;
        if !first_fixed && !second_fixed {
            both += 1;
        } else if !first_fixed {
            first += 1;
        } else if !second_fixed {
            second += 1;
        }
    }
    if first + both <= second {
        out_line!("First");
    } else if second + both < first {
        out_line!("Second");
    } else {
        out_line!("Tie");
    }
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
