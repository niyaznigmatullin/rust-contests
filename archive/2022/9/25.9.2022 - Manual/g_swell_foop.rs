//{"name":"g_swell_foop","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"g_swell_foop"}}}

use std::collections::HashSet;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let a = (0..5)
        .map(|_| input.read::<String>().into_bytes())
        .collect::<Vec<_>>();
    out!(format!("Case {}: ", _test_case));
    let mut used = HashSet::new();
    if go(a, &mut used) {
        out_line!("Yes");
    } else {
        out_line!("No");
    }
}

fn clear(a: &mut Vec<Vec<u8>>, row: usize, col: usize) -> usize {
    let mut res = 1;
    let ch = a[row][col];
    a[row][col] = b'.';
    for dx in -1..=1i32 {
        for dy in -1..=1i32 {
            if dx.abs() + dy.abs() != 1 {
                continue;
            }
            let new_row = row as i32 + dx;
            let new_col = col as i32 + dy;
            if new_row < 0 || new_col < 0 {
                continue;
            }
            let new_row = new_row as usize;
            let new_col = new_col as usize;
            if new_row >= a.len() || new_col >= a[new_row].len() || a[new_row][new_col] != ch {
                continue;
            }
            res += clear(a, new_row, new_col);
        }
    }
    res
}

fn move_down(a: &mut Vec<Vec<u8>>) {
    for row in (0..a.len()).rev() {
        for col in 0..a[row].len() {
            let ch = a[row][col];
            if ch != b'.' {
                a[row][col] = b'.';
                let mut new_row = row;
                while new_row + 1 < a.len() && a[new_row + 1][col] == b'.' {
                    new_row += 1;
                }
                a[new_row][col] = ch;
            }
        }
    }
}

fn move_left(a: &mut Vec<Vec<u8>>) {
    for col in 0..a[0].len() {
        for new_col in 0..col {
            if a.iter().all(|x| x[new_col] == b'.') {
                for row in 0..a.len() {
                    a[row][new_col] = a[row][col];
                    a[row][col] = b'.';
                }
                break;
            }
        }
    }
}

fn no_way(a: &Vec<Vec<u8>>) -> bool {
    let mut count = vec![0; 4];
    for x in a {
        for y in x {
            if *y == b'.' {
                continue;
            }
            let letter = "RGBY".as_bytes().iter().position(|e| *e == *y).unwrap();
            count[letter] += 1;
        }
    }
    count.iter().any(|x| *x == 1)
}

fn go(a: Vec<Vec<u8>>, used: &mut HashSet<Vec<Vec<u8>>>) -> bool {
    if a.iter().all(|x| x.into_iter().all(|y| *y == b'.')) {
        return true;
    }
    if used.contains(&a) || no_way(&a) {
        return false;
    }
    for row in 0..a.len() {
        for col in 0..a[row].len() {
            if a[row][col] == b'.' {
                continue;
            }
            let mut b = a.clone();
            if clear(&mut b, row, col) > 1 {
                move_down(&mut b);
                move_left(&mut b);
                if go(b, used) {
                    return true;
                }
            }
        }
    }
    used.insert(a);
    false
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
