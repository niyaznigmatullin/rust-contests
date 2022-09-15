//{"name":"E. Долгий путь домой","group":"Codeforces - Codeforces Round #816 (Div. 2)","url":"https://codeforces.com/contest/1715/problem/E?locale=ru","interactive":false,"timeLimit":3000,"tests":[{"input":"3 1 2\n1 3 1\n","output":"0 1 1\n"},{"input":"4 3 1\n1 2 3\n2 4 5\n3 4 7\n","output":"0 1 4 6\n"},{"input":"2 1 1\n2 1 893746473\n","output":"0 1\n"},{"input":"5 5 2\n2 1 33\n1 5 93\n5 3 48\n2 3 21\n4 2 1\n","output":"0 1 2 2 3\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"EDolgiiPutDomoi"}}}

use std::cmp::min;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out, out_line};
use algo_lib::datastructures::convex_set::ConvexSet;
use algo_lib::graphs::dijkstra::{shortest_path_sparse_dijkstra, shortest_path_sparse_dijkstra_continue};

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read();
    let m = input.read();
    let k = input.read();
    let mut edges = vec![Vec::new(); n];
    for _ in 0..m {
        let v = input.read::<usize>() - 1;
        let u = input.read::<usize>() - 1;
        let w = input.read();
        edges[v].push((u, w));
        edges[u].push((v, w));
    }
    let mut d = shortest_path_sparse_dijkstra(&edges, 0);
    for _ in 0..k {
        let mut lines = Vec::new();
        for (v, &dist) in (0..n).zip(d.iter()) {
            if dist != i64::MAX {
                let x = v as i64;
                lines.push((2 * x, -dist - x * x));
            }
        }
        let convex = ConvexSet::new(&lines);
        let mut replier = convex.monotonic_query();
        for i in 0..n {
            let x = i as i64;
            let best_line = replier.query(x);
            let value = -(best_line.0 * x + best_line.1) + x * x;
            d[i] = value;
        }
        d = shortest_path_sparse_dijkstra_continue(&edges, d);
    }
    out_line!(d);
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
