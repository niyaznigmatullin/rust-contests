//{"name":"F. 3-Сумма","group":"Codeforces - Codeforces Round #799 (Div. 4)","url":"https://codeforces.com/contest/1692/problem/F","interactive":false,"timeLimit":1000,"tests":[{"input":"6\n4\n20 22 19 84\n4\n1 11 1 2022\n4\n1100 1100 1100 1111\n5\n12 34 56 78 90\n4\n1 9 8 4\n6\n16 38 94 25 18 99\n","output":"YES\nYES\nNO\nNO\nYES\nYES\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"F3Summa"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read();
    let mut count = vec![0; 10];
    for _ in 0..n {
        let x: usize = input.read();
        count[x % 10] += 1;
    }
    let mut used = vec![0; 10];
    for d1 in 0..10 {
        if used[d1] + 1 > count[d1] {
            continue;
        }
        used[d1] += 1;
        for d2 in 0..10 {
            if used[d2] + 1 > count[d2] {
                continue;
            }
            used[d2] += 1;
            for d3 in [(23 - d1 - d2) % 10] {
                if used[d3] + 1 > count[d3] {
                    continue;
                }
                out_line!("YES");
                return;
            }
            used[d2] -= 1;
        }
        used[d1] -= 1;
    }
    out_line!("NO");
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
