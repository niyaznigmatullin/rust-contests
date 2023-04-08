//{"name":"G. Поднятие рейтинга","group":"Codeforces - Codeforces Round #839 (Div. 3)","url":"https://codeforces.com/contest/1772/problem/G","interactive":false,"timeLimit":2000,"tests":[{"input":"3\n7 2 10\n3 1 9 2 5 20 8\n7 1 10\n3 1 9 2 5 20 8\n5 10 12\n100 1 200 11 300\n","output":"20\n-1\n2\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"GPodnyatieReitinga"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read();
    let (starting_rating, required_rating): (i64, i64) = input.read();
    let a = {
        let mut a = input.read_vec::<i64>(n);
        a.sort();
        a
    };
    let mut rating = starting_rating;
    let mut rounds = 0;
    let mut wins = 0;
    for &x in &a {
        if rating >= x {
            rating += 1;
            wins += 1;
        } else {
            rating -= 1;
        }
        rounds += 1;
        if rating == required_rating {
            out_line!(rounds);
            return;
        }
    }
    if rating <= starting_rating {
        out_line!(-1);
        return;
    }
    loop {
        let next = if wins == n || a[wins] > required_rating {
            required_rating
        } else {
            a[wins]
        };
        let current_max = rating + (wins as i64);
        let diff = wins - (n - wins);
        let play_cycles = (next - current_max) / (diff as i64);
        // println!("{} {} {} {} {} {}", rating, next, rounds, current_max, diff, play_cycles);
        rounds += (n as i64) * play_cycles;
        rating += play_cycles * (diff as i64);

        if rating + (wins as i64) >= required_rating {
            rounds += required_rating - rating;
            rating = required_rating;
            break;
        } else {
            rounds += wins as i64;
            rating += wins as i64;
            while wins < n && rating >= a[wins] {
                rating += 1;
                rounds += 1;
                wins += 1;
                if rating == required_rating {
                    break;
                }
            }
            if rating == required_rating {
                break;
            }
            rating -= (n - wins) as i64;
            rounds += (n - wins) as i64;
        }
    }
    out_line!(rounds);
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
