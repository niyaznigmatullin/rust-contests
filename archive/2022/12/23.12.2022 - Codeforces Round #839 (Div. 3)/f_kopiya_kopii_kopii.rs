//{"name":"F. Копия копии копии","group":"Codeforces - Codeforces Round #839 (Div. 3)","url":"https://codeforces.com/contest/1772/problem/F","interactive":false,"timeLimit":2000,"tests":[{"input":"3 3 1\n\n010\n111\n010\n\n010\n101\n010\n","output":"2\n2\n1 2 2\n2 1\n"},{"input":"4 5 3\n\n00000\n01000\n11100\n01000\n\n00000\n01000\n10100\n01000\n\n00000\n01010\n10100\n01000\n\n00000\n01000\n10100\n01000\n","output":"3\n5\n1 2 4\n2 2\n2 4\n1 3 2\n2 1\n"},{"input":"5 3 0\n\n110\n010\n001\n011\n001\n","output":"1\n0\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"FKopiyaKopiiKopii"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let (n, _, k): (usize, usize, usize) = input.read();
    let mut fields = Vec::new();
    for _ in 0..=k {
        let mut field = vec![vec![]; n];
        for row in 0..n {
            field[row] = input.read::<String>().into_bytes();
        }
        fields.push(field);
    }
    for first in 0..=k {
        if let Some(answer) = get_ans(&fields, first) {
            out_line!(first + 1);
            out_line!(answer.len());
            for op in answer {
                match op {
                    Operation::Repaint(x, y) => {
                        out_line!(1, x + 1, y + 1);
                    }
                    Operation::Copy(index) => {
                        out_line!(2, index + 1);
                    }
                }
            }
            return;
        }
    }
    assert!(false);
}

fn get_ans(fields: &Vec<Vec<Vec<u8>>>, first: usize) -> Option<Vec<Operation>> {
    let mut others = Vec::new();
    for other in 0..fields.len() {
        if other == first {
            continue;
        }
        let dist = get_dist(&fields[first], &fields[other]);
        others.push((dist, &fields[other], other));
    }
    others.sort_by_key(|(d, _, _)| *d);
    let mut last = &fields[first];
    let mut last_diff = 0;
    let mut ans = Vec::new();
    for (cur_diff, cur, index) in others {
        if get_dist(last, cur) != cur_diff - last_diff {
            return None;
        }
        if let Some(ops) = can_make(last, cur) {
            for op in ops {
                ans.push(op);
            }
            ans.push(Operation::Copy(index));
        } else {
            return None;
        }
        last = cur;
        last_diff = cur_diff;
    }
    return Some(ans);
}

fn can_make(f: &Vec<Vec<u8>>, g: &Vec<Vec<u8>>) -> Option<Vec<Operation>> {
    let rows = f.len();
    let columns = f[0].len();
    let mut ans = Vec::new();
    for row in 0..rows {
        for column in 0..columns {
            if f[row][column] == g[row][column] {
                continue;
            }
            if row == 0 || column == 0 || row + 1 == rows || columns + 1 == columns {
                return None;
            }
            for (dr, dc) in [(-1, 0), (0, 1), (1, 0), (0, -1)] {
                let n_row = (row as i32 + dr) as usize;
                let n_column = (column as i32 + dc) as usize;
                if f[n_row][n_column] != g[n_row][n_column] || f[n_row][n_column] == f[row][column]
                {
                    return None;
                }
            }
            ans.push(Operation::Repaint(row, column));
        }
    }
    return Some(ans);
}

enum Operation {
    Repaint(usize, usize),
    Copy(usize),
}

fn get_dist(f: &Vec<Vec<u8>>, g: &Vec<Vec<u8>>) -> usize {
    f.into_iter()
        .zip(g)
        .map(|(row1, row2)| {
            row1.into_iter()
                .zip(row2)
                .map(|(cell1, cell2)| if *cell1 != *cell2 { 1 } else { 0 })
                .sum::<usize>()
        })
        .sum::<usize>()
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
