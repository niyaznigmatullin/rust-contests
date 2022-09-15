//{"name":"G. Посчитай поезда","group":"Codeforces - Codeforces Round #797 (Div. 3)","url":"https://codeforces.com/contest/1690/problem/G","interactive":false,"timeLimit":2000,"tests":[{"input":"3\n\n4 2\n6 2 3 7\n3 2\n4 7\n\n5 4\n10 13 5 2 6\n2 4\n5 2\n1 5\n3 2\n\n13 4\n769 514 336 173 181 373 519 338 985 709 729 702 168\n12 581\n6 222\n7 233\n5 117\n","output":"3 4\n4 4 2 3\n5 6 6 5\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"GPoschitaiPoezda"}}}

use std::collections::BTreeSet;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read();
    let m = input.read();
    let mut a = input.read_vec(n);
    let mut record_positions = BTreeSet::new();
    let mut best = u32::MAX;
    for (i, &v) in a.iter().enumerate() {
        if v < best {
            best = v;
            record_positions.insert(i);
        }
    }
    let mut ans = Vec::new();
    for _ in 0..m {
        let pos = input.read::<usize>() - 1;
        let subtract = input.read::<u32>();
        let last_value = a[*record_positions.range(0..pos + 1).rev().next().unwrap()];
        a[pos] -= subtract;
        if last_value > a[pos] {
            let mut to_remove = Vec::new();
            for &i in record_positions.range(pos + 1..n) {
                if a[i] < a[pos] {
                    break;
                }
                to_remove.push(i);
            }
            for i in to_remove {
                record_positions.remove(&i);
            }
            record_positions.insert(pos);
        }
        ans.push(record_positions.len());
    }
    out_line!(ans.into_iter().map(|x| x.to_string()).collect::<Vec<_>>().join(" "));
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
