//{"name":"A: Fourth Player","group":"Meta Coding Competitions - Meta Hacker Cup 2022 Round 3","url":"https://www.facebook.com/codingcompetitions/hacker-cup/2022/round-3/problems/A","interactive":false,"timeLimit":360000,"tests":[{"input":"5\n8\n1 2\n3 7\n6 8\n4 5\n16\n14 13 12 1\n15 5 6 7\n16 2 3 4\n8 9 10 11\n16\n15 13 11 6\n16 12 1 2\n5 9 7 8\n14 10 3 4\n8\n5 2\n8 4\n7 3\n6 1\n24\n7 6 14 22 18 12\n20 23 13 8 16 11\n24 21 4 9 1 19\n15 5 10 17 3 2\n","output":"Case #1: 2\nCase #2: 3\nCase #3: 0\nCase #4: 1\nCase #5: 3\n"}],"testType":"multiNumber","input":{"type":"regex","fileName":null,"pattern":"fourth_player_.*input[.]txt"},"output":{"type":"file","fileName":"fourth_player_output.txt","pattern":null},"languages":{"java":{"taskClass":"AFourthPlayer"}}}

use std::collections::BTreeSet;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    eprintln!("Running test {}", _test_case);
    let n = input.read();
    let mut whose = vec![0; n];
    let mut hands = vec![BTreeSet::new(); 4];
    for player in 0..4 {
        for _ in 0..n/4 {
            let card = input.read::<usize>() - 1;
            whose[card] = player;
            hands[player].insert(card);
        }
    }
    let mut first_team = 0;
    let mut all = 0;
    for card in (0..n).rev() {
        if 4 * all == n {
            break;
        }
        let player = whose[card];
        let mut wins = true;
        for other in player..4 {
            if (other & 1) == (player & 1) {
                continue;
            }
            if let Some(&beats) = hands[other].range(card..).next() {
                hands[other].remove(&beats);
                wins = false;
                break;
            }
        }
        if wins {
            all += 1;
            if (player & 1) == 0 {
                first_team += 1;
            }
        }
    }
    out_line!(format!("Case #{}: {}", _test_case, first_team));
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
