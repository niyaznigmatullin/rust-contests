//{"name":"F. Влад и отложенные дела","group":"Codeforces - Codeforces Round #787 (Div. 3)","url":"https://codeforces.com/contest/1675/problem/F","interactive":false,"timeLimit":2000,"tests":[{"input":"3\n\n3 1\n1 3\n2\n1 3\n1 2\n\n6 4\n3 5\n1 6 2 1\n1 3\n3 4\n3 5\n5 6\n5 2\n\n6 2\n3 2\n5 3\n1 3\n3 4\n3 5\n5 6\n5 2\n","output":"3\n7\n2\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"FVladIOtlozhennieDela"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read();
    let k = input.read();
    let vlad_house = input.read::<usize>() - 1;
    let nastya_house = input.read::<usize>() - 1;
    let mut to_visit = input.read_vec::<usize>(k);
    for x in &mut to_visit {
        *x -= 1;
    }
    let mut edges = vec![vec![]; n];
    for _ in 0..n - 1 {
        let (mut u, mut v) = input.read::<(usize, usize)>();
        u -= 1;
        v -= 1;
        edges[u].push(v);
        edges[v].push(u);
    }
    let mut parents = vec![0; n];
    let mut depth = vec![0; n];
    dfs(vlad_house, vlad_house, 0, &edges, &mut parents, &mut depth);
    let mut used = vec![false; n];
    for &x in to_visit.iter().chain([nastya_house].iter()) {
        let mut v = x;
        while !used[v] {
            used[v] = true;
            v = parents[v];
        }
    }
    let all_vertices = used.iter().filter(|&&x| x).count();
    out_line!((all_vertices - 1) * 2 - depth[nastya_house]);
}

fn dfs(v: usize, parent: usize, d: usize, edges: &Vec<Vec<usize>>, parents: &mut Vec<usize>, depth: &mut Vec<usize>) {
    parents[v] = parent;
    depth[v] = d;
    for &to in &edges[v] {
        if to != parent {
            dfs(to, v, d + 1, edges, parents, depth);
        }
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
