//{"name":"D2: First Time - Chapter 2","group":"Meta Coding Competitions - Meta Hacker Cup 2022 Round 3","url":"https://www.facebook.com/codingcompetitions/hacker-cup/2022/round-3/problems/D2","interactive":false,"timeLimit":360000,"tests":[{"input":"3\n5 4\n2 1\n3 4\n1 4\n5 4\n3 4\n1 2\n2 1\n1 3\n3 1\n8 6\n1 4\n2 3\n4 3\n8 7\n6 5\n7 5\n","output":"Case #1: 13\nCase #2: 4\nCase #3: 6\n"}],"testType":"multiNumber","input":{"type":"regex","fileName":null,"pattern":"first_time_chapter__.*input[.]txt"},"output":{"type":"file","fileName":"first_time_chapter__output.txt","pattern":null},"languages":{"java":{"taskClass":"D2FirstTimeChapter2"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out, out_line};
use std::collections::{BTreeSet, HashSet};
use std::mem;

fn solve(input: &mut Input, _test_case: usize) {
    eprintln!("Running test {}", _test_case);
    let (n, m): (usize, usize) = input.read();
    let colorings = input
        .read_vec::<(usize, usize)>(m)
        .into_iter()
        .map(|(x, y)| (x - 1, y - 1))
        .collect::<Vec<_>>();
    let edges = (0..n - 1).zip(1..n).collect::<Vec<_>>();
    let mut sets = vec![HashSet::new(); n];
    for i in 0..edges.len() {
        let e = &edges[i];
        sets[e.0].insert(i);
        sets[e.1].insert(i);
    }
    let mut when_satisfied = vec![m; edges.len()];
    for ((a, b), i) in colorings.into_iter().zip(0..m) {
        let mut v = a;
        let mut u = b;
        if sets[v].len() > sets[u].len() {
            let t = v;
            v = u;
            u = t;
        }
        let mut from = HashSet::new();
        let mut to = HashSet::new();
        mem::swap(&mut from, &mut sets[v]);
        mem::swap(&mut to, &mut sets[u]);
        for x in from {
            if to.contains(&x) {
                to.remove(&x);
                when_satisfied[x] = i;
            } else {
                to.insert(x);
            }
        }
        mem::swap(&mut to, &mut sets[b]);
    }
    let mut all_constraints = BTreeSet::new();
    for i in 0..edges.len() {
        all_constraints.insert((when_satisfied[i], i));
    }
    let mut ans = vec![0; n];
    for k in 1..=n {
        let mut removed = Vec::new();
        for z in 1..=((n - 1) / k) {
            let id = z * k - 1;
            let to_remove = (when_satisfied[id], id);
            removed.push(to_remove);
            all_constraints.remove(&to_remove);
        }
        let answer = if let Some(&last) = all_constraints.iter().rev().next() {
            last.0 + 1
        } else {
            0
        };
        ans[k - 1] = answer;
        for x in removed {
            all_constraints.insert(x);
        }
    }
    let mut sum = 0i64;
    for x in ans {
        sum += if x == m + 1 { -1i64 } else { x as i64 };
    }
    out_line!(format!("Case #{}: {}", _test_case, sum));
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
