//{"name":"H. Азартные игры","group":"Codeforces - Codeforces Round #799 (Div. 4)","url":"https://codeforces.com/contest/1692/problem/H","interactive":false,"timeLimit":2000,"tests":[{"input":"4\n5\n4 4 3 4 4\n5\n11 1 11 1 11\n1\n1000000000\n10\n8 8 8 9 9 6 6 9 6 6\n","output":"4 1 5\n1 2 2\n1000000000 1 1\n6 6 10\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"HAzartnieIgri"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read();
    let a: Vec<_> = (0..n).map(|_| input.read::<u32>()).collect();
    let mut all = a.clone();
    all.sort();
    all.dedup();
    let a: Vec<_> = a.iter().map(|x| all.binary_search(x).unwrap()).collect();
    let mut positions = vec![Vec::new(); all.len()];
    for (i, &x) in a.iter().enumerate() {
        positions[x].push(i);
    }
    let mut best_value = i32::MIN;
    let mut ans = (0, 0, 0);
    for (v, p) in positions.iter().enumerate() {
        let mut best_met_value = i32::MIN;
        let mut best_met_i = 0;
        for (pi, &i) in p.iter().enumerate() {
            let cur_met_value = (2 * pi) as i32 - i as i32;
            if best_met_value < -cur_met_value {
                best_met_value = -cur_met_value;
                best_met_i = i;
            }
            if best_met_value + cur_met_value + 1 > best_value {
                best_value = best_met_value + cur_met_value + 1;
                ans = (v, best_met_i, i);
            }
        }
    }
    out_line!(all[ans.0], ans.1 + 1, ans.2 + 1);
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
