//{"name":"G. Любимая задача SlavicG-а","group":"Codeforces - Codeforces Round #835 (Div. 4)","url":"https://codeforces.com/contest/1760/problem/G","interactive":false,"timeLimit":2000,"tests":[{"input":"3\n5 1 4\n1 3 1\n2 3 2\n4 3 3\n3 5 1\n2 1 2\n1 2 2\n6 2 3\n1 2 1\n2 3 1\n3 4 1\n4 5 3\n5 6 5\n","output":"YES\nNO\nYES\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"GLyubimayaZadachaSlavicGA"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out, out_line};
use std::collections::HashSet;

#[derive(Copy, Clone)]
struct Edge {
    to: usize,
    w: u32,
}

fn dfs(v: usize, p: usize, xr: &mut Vec<u32>, edges: &Vec<Vec<Edge>>, invalid: usize) {
    for e in &edges[v] {
        if e.to == p || e.to == invalid {
            continue;
        }
        xr[e.to] = xr[v] ^ e.w;
        dfs(e.to, v, xr, edges, invalid);
    }
}

fn solve(input: &mut Input, _test_case: usize) {
    let (n, mut a, mut b) : (usize, usize, usize) = input.read();
    a -= 1;
    b -= 1;
    let mut edges = vec![vec![]; n];
    for _ in 1..n {
        let u = input.read::<usize>() - 1;
        let v = input.read::<usize>() - 1;
        let w = input.read();
        edges[u].push(Edge { to: v, w });
        edges[v].push(Edge { to: u, w });
    }
    let mut from_a = vec![0u32; n];
    let mut from_b = vec![0u32; n];
    dfs(a, a, &mut from_a, &edges, b);
    dfs(b, b, &mut from_b, &edges, b);
    let mut possible = HashSet::new();
    for i in 0..n {
        if i != b {
            possible.insert(from_a[i]);
        }
    }
    let ans = (0..n).any(|i| {
        if i == b {
            return false;
        }
        possible.contains(&from_b[i])
    });
    out_line!(if ans { "YES" } else { "NO" });
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
