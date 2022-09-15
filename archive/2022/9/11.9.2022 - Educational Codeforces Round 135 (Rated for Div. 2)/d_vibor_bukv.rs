//{"name":"D. Выбор букв","group":"Codeforces - Educational Codeforces Round 135 (Rated for Div. 2)","url":"https://codeforces.com/contest/1728/problem/D","interactive":false,"timeLimit":2000,"tests":[{"input":"2\nforces\nabba\n","output":"Alice\nDraw\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DViborBukv"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out, out_line};
use std::cmp::{max, min};

fn solve(input: &mut Input, _test_case: usize) {
    let s = input.read::<String>().into_bytes();
    let n = s.len();
    let mut dp = vec![vec![0; n]; n];
    for left in 0..n - 1 {
        dp[left][left + 1] = if s[left] != s[left + 1] { 1 } else { 0 };
    }
    for left in (0..n).rev() {
        for right in (left + 3..n).step_by(2) {
            let mut alice_can_achieve = -1;
            for move_alice in 0..2 {
                let left1 = left + (1 - move_alice);
                let right1 = right - move_alice;
                let letter_alice = if move_alice == 0 { s[left] } else { s[right] };
                let mut bob_can_achieve = 1;
                for move_bob in 0..2 {
                    let left2 = left1 + (1 - move_bob);
                    let right2 = right1 - move_bob;
                    let letter_bob = if move_bob == 0 { s[left1] } else { s[right1] };
                    let mut value = match dp[left2][right2] {
                        0 => {
                            if letter_bob > letter_alice {
                                1
                            } else if letter_bob < letter_alice {
                                -1
                            } else {
                                0
                            }
                        }
                        x => x,
                    };
                    bob_can_achieve = min(bob_can_achieve, value);
                }
                alice_can_achieve = max(alice_can_achieve, bob_can_achieve);
            }
            dp[left][right] = alice_can_achieve;
        }
    }
    out_line!(match dp[0][n - 1] {
        1 => "Alice",
        0 => "Draw",
        _ => "Bob",
    });
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
