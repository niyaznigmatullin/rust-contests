//{"name":"C. Восстановление длительности выполнения заданий","group":"Codeforces - Codeforces Round #797 (Div. 3)","url":"https://codeforces.com/contest/1690/problem/C","interactive":false,"timeLimit":2000,"tests":[{"input":"4\n3\n0 3 7\n2 10 11\n2\n10 15\n11 16\n9\n12 16 90 195 1456 1569 3001 5237 19275\n13 199 200 260 9100 10000 10914 91066 5735533\n1\n0\n1000000000\n","output":"2 7 1\n1 1\n1 183 1 60 7644 900 914 80152 5644467\n1000000000\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CVosstanovlenieDlitelnostiVipolneniyaZadanii"}}}

use std::cmp::max;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read();
    let s = input.read_vec::<u32>(n);
    let t = input.read_vec::<u32>(n);
    let mut cur_time = 0;
    let mut d = vec![0; n];
    for i in 0..n {
        d[i] = t[i] - max(cur_time, s[i]);
        cur_time = t[i];
    }
    out_line!(d.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(" "));
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
