//{"name":"B. Палиндромные числа","group":"Codeforces - Codeforces Round #802 (Div. 2)","url":"https://codeforces.com/contest/1700/problem/B","interactive":false,"timeLimit":2000,"tests":[{"input":"3\n2\n99\n4\n1023\n3\n385\n","output":"32\n8646\n604\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BPalindromnieChisla"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out, out_line};
fn relax(a: &mut Vec<i32>) {
    let mut carry = 0;
    for i in a.iter_mut() {
        *i += carry;
        carry = *i / 10;
        *i %= 10;
    }
    if carry > 0 {
        a.push(carry);
    }
}

fn solve(input: &mut Input, _test_case: usize) {
    let _ : usize = input.read();
    let str = input.read::<String>();
    let s = str.into_bytes().iter().rev().map(|&c| (c - b'0') as i32).collect::<Vec<_>>();
    let mut a = s.clone();
    *a.last_mut().unwrap() += 1;
    relax(&mut a);
    let mut success = true;
    let n = a.len();
    for i in (0..n / 2).rev() {
        if a[i] > a[n - i - 1] {
            success = false;
            break;
        }
        if a[i] < a[n - i - 1] {
            for j in (0..i + 1).rev() {
                a[j] = a[n - j - 1];
            }
            break;
        }
    }
    if !success {
        a[n / 2] += 1;
        relax(&mut a);
        for i in 0..n / 2 {
            a[i] = a[n - i - 1];
        }
    }
    sub(&mut a, &s);
    out_line!(a.iter().rev().map(|x| x.to_string()).collect::<Vec<_>>().join(""));
}

fn sub(a: &mut Vec<i32>, b: &Vec<i32>) {
    let mut need = 0;
    for i in 0..a.len() {
        let to_sub = if i < b.len() {
            b[i]
        } else {
            0
        };
        a[i] -= to_sub;
        a[i] -= need;
        need = 0;
        if a[i] < 0 {
            a[i] += 10;
            need += 1;
        }
    }
    while a[a.len() - 1] == 0 {
        a.pop();
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
