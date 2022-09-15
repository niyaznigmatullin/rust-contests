//{"name":"G. Удали ориентированные ребра","group":"Codeforces - Codeforces Round #786 (Div. 3)","url":"https://codeforces.com/contest/1674/problem/G","interactive":false,"timeLimit":2000,"tests":[{"input":"3 3\n1 2\n2 3\n1 3\n","output":"2\n"},{"input":"5 0\n","output":"1\n"},{"input":"7 8\n7 1\n1 3\n6 2\n2 3\n7 2\n2 4\n7 3\n6 3\n","output":"3\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"GUdaliOrientirovannieRebra"}}}

use std::cmp::max;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out, out_line};
use algo_lib::graphs::topsort::TopSortGraph;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read();
    let m = input.read();
    let mut edges = input.read_vec::<(usize, usize)>(m);
    for (from, to) in &mut edges {
        *from -= 1;
        *to -= 1;
    }
    let mut incoming = vec![0; n];
    let mut outcoming = vec![0; n];
    for &(from, to) in &edges {
        incoming[to] += 1;
        outcoming[from] += 1;
    }
    let t_graph = TopSortGraph::new(n, &edges);
    let sorted_vertices = t_graph.get_top_sort();
    let mut f = vec![0; n];
    for &v in &sorted_vertices {
        if outcoming[v] < 2 {
            continue;
        }
        for &to in t_graph.edges(v) {
            if incoming[to] < 2 {
                continue;
            }
            f[to] = max(f[v] + 1, f[to]);
        }
    }
    let ans = *f.iter().max().unwrap();
    out_line!(ans + 1);
}

pub(crate) fn run(mut input: Input) -> bool {
    solve(&mut input, 1);
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
