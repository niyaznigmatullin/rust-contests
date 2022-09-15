//{"name":"F. Съедая конфеты","group":"Codeforces - Codeforces Round #784 (Div. 4)","url":"https://codeforces.com/contest/1669/problem/F","interactive":false,"timeLimit":1000,"tests":[{"input":"4\n3\n10 20 10\n6\n2 1 4 2 4 1\n5\n1 2 4 8 16\n9\n7 3 20 5 15 1 11 8 10\n","output":"2\n6\n0\n7\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"FSedayaKonfeti"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read();
    let w = input.read_vec::<u32>(n);
    let mut sum1 = 0;
    let mut sum2 = 0;
    let mut ans = 0;
    let mut j = n;
    for i in 0..n {
        sum1 += w[i];
        while j > 0 && sum2 < sum1 {
            j -= 1;
            sum2 += w[j];
        }
        if j <= i {
            break;
        }
        if sum1 == sum2 {
            ans = (i + 1) + (n - j);
        }
    }
    out_line!(ans);
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
