//{"name":"E. Максимизация цен","group":"Codeforces - Codeforces Round #797 (Div. 3)","url":"https://codeforces.com/contest/1690/problem/E","interactive":false,"timeLimit":2000,"tests":[{"input":"6\n6 3\n3 2 7 1 4 8\n4 3\n2 1 5 6\n4 12\n0 0 0 0\n2 1\n1 1\n6 10\n2 0 0 5 9 4\n6 5\n5 3 8 6 3 2\n","output":"8\n4\n0\n2\n1\n5\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"EMaksimizatsiyaTsen"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read();
    let k = input.read();
    let a = input.read_vec::<usize>(n);
    let mut count_mods = vec![0; k];
    let mut ans = 0;
    for &x in &a {
        count_mods[x % k] += 1;
        ans += x / k;
    }
    {
        let mut j = k - 1;
        for i in 1..k {
            while count_mods[i] > 0 {
                while j > 0 && count_mods[j] == 0 {
                    j -= 1;
                }
                if i + j < k || (i == j && count_mods[i] < 2) {
                    break;
                }
                count_mods[i] -= 1;
                count_mods[j] -= 1;
                ans += 1;
            }
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
