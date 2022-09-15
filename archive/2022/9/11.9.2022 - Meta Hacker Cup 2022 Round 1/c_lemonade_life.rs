//{"name":"C: Lemonade Life","group":"Meta Coding Competitions - Meta Hacker Cup 2022 Round 1","url":"https://www.facebook.com/codingcompetitions/hacker-cup/2022/round-1/problems/C","interactive":false,"timeLimit":360000,"tests":[{"input":"5\n9 25 8\n0 5\n1 6\n6 3\n6 7\n3 4\n9 2\n2 1\n1 2\n11 8\n3 100 7\n0 0\n4 1\n7 2\n3 100 7\n0 0\n4 1\n8 2\n6 0 1000000000\n0 10\n2 5\n1 7\n7 4\n8 1\n10 0\n12 1600 2000\n0 30\n16 48\n36 57\n951 45\n397 63\n447 63\n185 16\n362 10\n432 9\n507 11\n643 16\n1000 30\n","output":"Case #1: 115\nCase #2: 200\nCase #3: -1\nCase #4: 56\nCase #5: 184654\n"}],"testType":"multiNumber","input":{"type":"regex","fileName":null,"pattern":"lemonade_life_.*input[.]txt"},"output":{"type":"file","fileName":"lemonade_life_output.txt","pattern":null},"languages":{"java":{"taskClass":"CLemonadeLife"}}}

use algo_lib::geometry::algorithms::convex_hull;
use algo_lib::geometry::point_32::Point32;
use algo_lib::graphs::dijkstra::shortest_path_dijkstra_by;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out_line, out};
use std::cmp::max;

fn solve(input: &mut Input, _test_case: usize) {
    let (n, k, d): (usize, i64, i64) = input.read();
    let d = d * d;
    let p = input.read_vec::<Point32>(n);
    let start = p[0];
    let finish = p[n - 1];
    let p = convex_hull(p);
    let source = p.iter().position(|x| *x == start).unwrap();
    let target = p.iter().position(|x| *x == finish).unwrap();
    let d = shortest_path_dijkstra_by(&p, source, |&from, &to| {
        let dist_squared = (from - to).len_squared();
        if dist_squared > d {
            None
        } else {
            Some(max(k, dist_squared))
        }
    });
    out_line!(format!(
        "Case #{}: {}",
        _test_case,
        if d[target] == i64::MAX { -1 } else { d[target] }
    ));
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
