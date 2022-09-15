//{"name":"D: Second Flight","group":"Meta Coding Competitions - Meta Hacker Cup 2022 Qualification Round","url":"https://www.facebook.com/codingcompetitions/hacker-cup/2022/qualification-round/problems/D","interactive":false,"timeLimit":360000,"tests":[{"input":"3\n4 5 6\n1 2 10\n1 3 5\n2 3 15\n2 4 10\n3 4 7\n1 2\n1 3\n2 3\n2 4\n3 4\n4 1\n4 3 6\n1 2 10\n2 3 20\n3 1 30\n1 2\n1 3\n1 4\n2 3\n2 4\n3 4\n4 3 6\n1 2 20\n2 3 10\n3 4 30\n1 2\n1 3\n1 4\n2 3\n2 4\n3 4\n","output":"Case #1: 25 20 42 27 24 15\nCase #2: 40 70 0 50 0 0\nCase #3: 40 10 0 20 10 60\n"}],"testType":"multiNumber","input":{"type":"regex","fileName":null,"pattern":"second_flight_.*input[.]txt"},"output":{"type":"file","fileName":"second_flight_output.txt","pattern":null},"languages":{"java":{"taskClass":"DSecondFlight"}}}

use std::cmp::{max, min};
use std::collections::HashMap;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let (n, m, q) = input.read();
    let mut edges = vec![HashMap::new(); n];
    for _ in 0..m {
        let a = input.read::<usize>() - 1;
        let b = input.read::<usize>() - 1;
        let c: i64 = input.read();
        *edges[a].entry(b).or_insert(0) += c;
        *edges[b].entry(a).or_insert(0) += c;
    }
    let mut cache = HashMap::new();
    let mut answer = Vec::new();
    for _ in 0..q {
        let x = input.read::<usize>() - 1;
        let y = input.read::<usize>() - 1;
        let current_answer = *cache.entry((min(x, y), max(x, y))).or_insert_with(|| {
            let edges = &edges;
            let mut answer = edges[x].get(&y).copied().unwrap_or(0) * 2;
            let (brute, ask) = if edges[x].len() < edges[y].len() {
                (&edges[x], &edges[y])
            } else {
                (&edges[y], &edges[x])
            };
            for (v, value) in brute {
                answer += min(ask.get(v).copied().unwrap_or(0), *value);
            }
            answer
        });
        answer.push(current_answer);
    }
    out_line!(format!("Case #{}: {}", _test_case, answer.into_iter().map(|x| x.to_string()).collect::<Vec<_>>().join(" ")));
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
