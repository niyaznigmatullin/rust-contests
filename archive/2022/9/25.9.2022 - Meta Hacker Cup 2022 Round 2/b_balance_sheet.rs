//{"name":"B: Balance Sheet","group":"Meta Coding Competitions - Meta Hacker Cup 2022 Round 2","url":"https://www.facebook.com/codingcompetitions/hacker-cup/2022/round-2/problems/B","interactive":false,"timeLimit":360000,"tests":[{"input":"4\n8 10\n1 2 10 20\n2 4 25 30\n1 4 45 40\n4 5 35 20\n1 5 10 15\n5 6 45 30\n6 7 30 40\n8 9 80 90\n8 6\n1 2 10 20\n2 4 25 30\n1 4 45 40\n4 5 35 20\n1 5 10 15\n5 6 45 30\n6 7 30 40\n8 9 80 90\n2 1\n1 2 10 20\n3 4 30 40\n2 1\n2 3 30 40\n1 2 10 20\n","output":"Case #1: 140\nCase #2: 135\nCase #3: 0\nCase #4: 10\n"}],"testType":"multiNumber","input":{"type":"regex","fileName":null,"pattern":"balance_sheet_.*input[.]txt"},"output":{"type":"file","fileName":"balance_sheet_output.txt","pattern":null},"languages":{"java":{"taskClass":"BBalanceSheet"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::math::modular::primitive::Modular;
use algo_lib::math::modular::primitive::ModularType;
use algo_lib::{out, out_line};
use std::cmp::Reverse;
use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::iter::once;

type Mod = ModularType<1000000007>;

struct Client {
    x: i64,
    y: i64,
}

fn solve(input: &mut Input, _test_case: usize) {
    let (n, k): (usize, usize) = input.read();
    let mut clients = Vec::new();
    let mut buy_clients = BTreeMap::new();
    let mut sell_clients = BTreeMap::new();
    for i in 0..n {
        let (a, b, x, y): (usize, usize, i64, i64) = input.read();
        clients.push(Client { x, y });
        buy_clients.entry(a).or_insert(Vec::new()).push(i);
        sell_clients.entry(b).or_insert(Vec::new()).push(i);
    }
    let mut best = vec![Vec::<i64>::new(); n];
    for (day, mut sell_clients) in sell_clients.into_iter().rev() {
        let mut buy_clients = buy_clients.get(&day).cloned().unwrap_or(Vec::new());
        buy_clients.sort_by_key(|&index| Reverse(clients[index].x));
        sell_clients.sort_by_key(|&index| Reverse(clients[index].y));
        let mut buy_clients = buy_clients.into_iter().peekable();
        let mut all = BTreeSet::new();
        let mut free = 0;
        for sell_to in sell_clients {
            let client = &clients[sell_to];
            while let Some(&buy_from) = buy_clients
                .peek()
                .filter(|buy_from| clients[**buy_from].x > client.y)
            {
                buy_clients.next();
                let client = &clients[buy_from];
                for &z in &best[buy_from] {
                    all.insert((z + client.x, free));
                    free += 1;
                }
            }
            best[sell_to] = all
                .iter()
                .rev()
                .take(k)
                .map(|(x, _)| *x - client.y)
                .chain(once(0))
                .collect();
            cut_to(k, &mut best[sell_to]);
        }
    }
    let mut all = best
        .iter()
        .map(|x| x.into_iter())
        .flatten()
        .copied()
        .collect::<Vec<_>>();
    cut_to(k, &mut all);
    let mut ans = Mod::from(0);
    for x in all {
        ans += Mod::from(x);
    }
    out_line!(format!("Case #{}: {}", _test_case, ans.value()));
}

fn cut_to(k: usize, all: &mut Vec<i64>) {
    all.sort();
    all.reverse();
    *all = all.iter().take(k).copied().collect();
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
