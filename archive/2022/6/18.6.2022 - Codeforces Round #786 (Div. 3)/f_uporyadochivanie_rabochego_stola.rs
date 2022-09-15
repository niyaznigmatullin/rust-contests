//{"name":"F. Упорядочивание рабочего стола","group":"Codeforces - Codeforces Round #786 (Div. 3)","url":"https://codeforces.com/contest/1674/problem/F","interactive":false,"timeLimit":3000,"tests":[{"input":"4 4 8\n..**\n.*..\n*...\n...*\n1 3\n2 3\n3 1\n2 3\n3 4\n4 3\n2 3\n2 2\n","output":"3\n4\n4\n3\n4\n5\n5\n5\n"},{"input":"2 5 5\n*...*\n*****\n1 3\n2 2\n1 3\n1 5\n2 3\n","output":"2\n3\n3\n3\n2\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"FUporyadochivanieRabochegoStola"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let (n, m, q) : (_, _, u32) = input.read();
    let mut field = Vec::new();
    let mut count = 0;
    for _ in 0..n {
        let row = input.read::<String>().into_bytes();
        let row = row.into_iter()
            .map(|x| x == b'*')
            .collect::<Vec<_>>();
        count += row.iter().filter(|&&x| x).count();
        field.push(row);
    }
    let mut xor_count = 0;
    for row in 0..n {
        for col in 0..m {
            if field[row][col] != inside(row, col, n, count) {
                xor_count += 1;
            }
        }
    }
    for _ in 0..q {
        let row = input.read::<usize>() - 1;
        let col = input.read::<usize>() - 1;
        if field[row][col] {
            let n_col = (count - 1) / n;
            let n_row = (count - 1) % n;
            if field[n_row][n_col] {
                xor_count += 1;
            } else {
                xor_count -= 1;
            }
            count -= 1;
        } else {
            let n_col = count / n;
            let n_row = count % n;
            if !field[n_row][n_col] {
                xor_count += 1;
            } else {
                xor_count -= 1;
            }
            count += 1;
        }
        if field[row][col] == inside(row, col, n, count) {
            xor_count += 1;
        } else {
            xor_count -= 1;
        }
        field[row][col] ^= true;
        out_line!(xor_count / 2);
    }
}

fn inside(row: usize, col: usize, columns_size: usize, count : usize) -> bool {
    col * columns_size + row < count
}

pub(crate) fn run(mut input: Input) -> bool {
    solve(&mut input, 1);
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
