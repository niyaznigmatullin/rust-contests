//{"name":"B2: Watering Well - Chapter 2","group":"Meta Coding Competitions - Meta Hacker Cup 2022 Round 1","url":"https://www.facebook.com/codingcompetitions/hacker-cup/2022/round-1/problems/B2","interactive":false,"timeLimit":360000,"tests":[{"input":"4\n2\n2 2\n5 5\n2\n2 5\n6 6\n4\n1 1\n4 3\n6 3\n6 5\n3\n3 1\n5 2\n6 5\n8\n2837 745\n62 1162\n2634 1112\n1746 2618\n847 127\n986 1993\n732 1273\n2003 1998\n4\n1276 2231\n1234 1234\n287 2371\n3000 3000\n5\n283746263 475619273\n987361523 361738847\n281936352 666152443\n143042069 482716253\n1000000000 100000000\n5\n0 0\n123456789 987654321\n192837465 918273645\n135792468 864209753\n703692581 185296307\n","output":"Case #1: 52\nCase #2: 131\nCase #3: 110090622\nCase #4: 391473143\n"}],"testType":"multiNumber","input":{"type":"regex","fileName":null,"pattern":"watering_well_chapter__.*input[.]txt"},"output":{"type":"file","fileName":"watering_well_chapter__output.txt","pattern":null},"languages":{"java":{"taskClass":"B2WateringWellChapter2"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out, out_line};

struct Solver {
    sum_squared: i32,
    sum: i32,
    count: i32,
}

impl Solver {
    fn new(a: Vec<i32>) -> Self {
        let sum_squared = a.iter().fold(0, |mut acc, val| {
            add(&mut acc, mul(*val, *val));
            acc
        });
        let sum = a.iter().fold(0, |mut acc, val| {
            add(&mut acc, *val);
            acc
        });
        Self {
            sum_squared,
            sum,
            count: a.len() as i32,
        }
    }

    fn query(&self, qx: i32) -> i32 {
        // sum((qx - x[i]) ^ 2) = sum(qx ^ 2 + x[i] ^ 2 - 2 * x[i] * qx) = cnt * qx^2 + sum_sq - 2 * qx * sum
        let mut ans = mul(self.count, mul(qx, qx));
        add(&mut ans, self.sum_squared);
        let sub = mul(qx, mul(self.sum, 2));
        add(&mut ans, MOD - sub);
        ans
    }
}

const MOD: i32 = 1000000007;

fn add(a: &mut i32, b: i32) {
    *a += b;
    if *a >= MOD {
        *a -= MOD;
    }
}

fn mul(a: i32, b: i32) -> i32 {
    (a as i64 * b as i64 % MOD as i64) as i32
}

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read();
    let (x, y): (Vec<_>, Vec<_>) = input.read_vec::<(i32, i32)>(n).into_iter().unzip();
    let solve_x = Solver::new(x);
    let solve_y = Solver::new(y);
    let q = input.read();
    let mut ans = 0;
    for _ in 0..q {
        let (qx, qy) = input.read();
        add(&mut ans, solve_x.query(qx));
        add(&mut ans, solve_y.query(qy));
    }
    out_line!(format!("Case #{}: {}", _test_case, ans));
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
