//{"name":"G. Вырежи подстроки","group":"Codeforces - Codeforces Round #820 (Div. 3)","url":"https://codeforces.com/contest/1729/problem/G","interactive":false,"timeLimit":2000,"tests":[{"input":"8\nabababacababa\naba\nddddddd\ndddd\nxyzxyz\nxyz\nabc\nabcd\nabacaba\nabaca\nabc\ndef\naaaaaaaa\na\naaaaaaaa\naa\n","output":"2 2\n1 4\n2 1\n0 1\n1 1\n0 1\n8 1\n3 6\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"GVirezhiPodstroki"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::strings::prefix_function::prefix_function;
use algo_lib::{out, out_line};

const MOD: i32 = 1000000007;

fn add(a: &mut i32, b: i32) {
    *a += b;
    if *a >= MOD {
        *a -= MOD;
    }
}

fn solve(input: &mut Input, _test_case: usize) {
    let s = input.read::<String>().into_bytes();
    let sub = input.read::<String>().into_bytes();
    let mut a = vec![0; s.len() + sub.len() + 1];
    a[..sub.len()].copy_from_slice(&sub);
    a[sub.len() + 1..].copy_from_slice(&s);
    let p = prefix_function(&a)
        .into_iter()
        .skip(sub.len() + 1)
        .collect::<Vec<_>>();
    let n = sub.len();
    let entries = (0..s.len())
        .into_iter()
        .filter(|&index| p[index] == n)
        .map(|index| index + 1 - n)
        .collect::<Vec<_>>();
    if entries.is_empty() {
        out_line!("0 1");
        return;
    }
    let mut dp = vec![(0, 0); entries.len()];
    for i in 0..entries.len() {
        let mut best = usize::MAX;
        let mut ways = 0;
        let mut k = i;
        for j in (0..i + 1).rev() {
            if entries[j] + n <= entries[i] {
                break;
            }
            while k > 0 && entries[k - 1] + n > entries[j] {
                k -= 1;
            }
            let old = if k == 0 {
                (0, 1)
            } else {
                dp[k - 1]
            };
            if best > old.0 + 1 {
                ways = old.1;
                best = old.0 + 1;
            } else if best == old.0 + 1 {
                add(&mut ways, old.1);
            }
        }
        dp[i] = (best, ways);
    }
    let ans = dp.last().copied().unwrap();
    out_line!(ans.0, ans.1);
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
