//{"name":"C. Где слон?","group":"Codeforces - Codeforces Round #799 (Div. 4)","url":"https://codeforces.com/contest/1692/problem/C","interactive":false,"timeLimit":1000,"tests":[{"input":"3\n\n.....#..\n#...#...\n.#.#....\n..#.....\n.#.#....\n#...#...\n.....#..\n......#.\n\n#.#.....\n.#......\n#.#.....\n...#....\n....#...\n.....#..\n......#.\n.......#\n\n.#.....#\n..#...#.\n...#.#..\n....#...\n...#.#..\n..#...#.\n.#.....#\n#.......\n","output":"4 3\n2 2\n4 5\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CGdeSlon"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let mut field = Vec::new();
    for _ in 0..8 {
        field.push(input.read::<String>().into_bytes());
    }
    let occupied = b'#';
    for row in 1..7 {
        for col in 1..7 {
            if field[row][col] == occupied && field[row - 1][col - 1] == occupied && field[row - 1][col + 1] == occupied &&
                field[row + 1][col - 1] == occupied && field[row + 1][col + 1] == occupied {
                out_line!(row + 1, col + 1);
                return;
            }
        }
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
