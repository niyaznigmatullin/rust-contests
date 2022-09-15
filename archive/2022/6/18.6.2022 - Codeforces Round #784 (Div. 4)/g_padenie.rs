//{"name":"G. Падение","group":"Codeforces - Codeforces Round #784 (Div. 4)","url":"https://codeforces.com/contest/1669/problem/G","interactive":false,"timeLimit":1000,"tests":[{"input":"3\n6 10\n.*.*....*.\n.*.......*\n...o....o.\n.*.*....*.\n..........\n.o......o*\n2 9\n...***ooo\n.*o.*o.*o\n5 5\n*****\n*....\n*****\n....*\n*****\n","output":"..........\n...*....*.\n.*.o....o.\n.*........\n.*......**\n.o.*....o*\n\n....**ooo\n.*o**o.*o\n\n.....\n*...*\n*****\n*****\n*****\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"GPadenie"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let rows = input.read();
    let cols = input.read();
    let mut field = Vec::new();
    for _ in 0..rows {
        field.push(input.read::<String>().into_bytes());
    }
    for row in (0..rows).rev() {
        for col in 0..cols {
            if field[row][col] == b'*' {
                let mut n_row = row;
                while n_row + 1 < rows && field[n_row + 1][col] == b'.' {
                    field[n_row + 1][col] = b'*';
                    field[n_row][col] = b'.';
                    n_row += 1;
                }
            }
        }
    }
    for row in field {
        out_line!(String::from_utf8(row).unwrap());
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
