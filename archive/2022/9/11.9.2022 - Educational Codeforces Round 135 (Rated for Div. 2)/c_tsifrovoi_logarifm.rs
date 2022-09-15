//{"name":"C. Цифровой логарифм","group":"Codeforces - Educational Codeforces Round 135 (Rated for Div. 2)","url":"https://codeforces.com/contest/1728/problem/C","interactive":false,"timeLimit":2000,"tests":[{"input":"4\n1\n1\n1000\n4\n1 2 3 4\n3 1 4 2\n3\n2 9 3\n1 100 9\n10\n75019 709259 5 611271314 9024533 81871864 9 3 6 4865\n9503 2 371245467 6 7 37376159 8 364036498 52295554 169\n","output":"2\n0\n2\n18\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CTsifrovoiLogarifm"}}}

use std::cmp::min;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read();
    let mut a = input.read_vec::<i32>(n);
    let mut b = input.read_vec::<i32>(n);
    a.sort();
    b.sort();
    let mut count_a = vec![0; 10];
    let mut count_b = vec![0; 10];
    let mut answer = 0i32;
    fn digits(mut n: i32) -> i32 {
        let mut ans = 0;
        while n > 0 {
            n /= 10;
            ans += 1;
        }
        ans
    }
    {
        let mut ia = a.into_iter().peekable();
        let mut ib = b.into_iter().peekable();
        while ia.peek().is_some() || ib.peek().is_some() {
            let b_next = ib.peek();
            let a_next = ia.peek();
            if let (Some(x), Some(y)) = (a_next, b_next) {
                if x == y {
                    ia.next();
                    ib.next();
                    continue;
                }
            }
            let (wh, value) = if b_next
                .map(|y| a_next.map(|x| x < y).unwrap_or(false))
                .unwrap_or(true)
            {
                (&mut count_a, ia.next().unwrap())
            } else {
                (&mut count_b, ib.next().unwrap())
            };
            if value >= 10 {
                answer += 1;
                wh[digits(value) as usize] += 1;
            } else {
                wh[value as usize] += 1;
            }
        }
    }
    for i in 2..10 {
        let take = min(count_a[i], count_b[i]);
        answer += count_b[i] + count_a[i] - 2 * take;
    }
    out_line!(answer);
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
