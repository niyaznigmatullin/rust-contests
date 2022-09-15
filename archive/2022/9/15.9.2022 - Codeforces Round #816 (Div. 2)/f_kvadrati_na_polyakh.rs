//{"name":"F. Квадраты на полях","group":"Codeforces - Codeforces Round #816 (Div. 2)","url":"https://codeforces.com/contest/1715/problem/F?locale=ru","interactive":true,"timeLimit":1000,"tests":[{"input":"3 3\n\n\n\n\n\n0.5\n\n\n\n\n\n0.5\n","output":"? 4\n0 0\n2 0\n2 3\n0 3\n\n? 4\n0 0\n0 1\n3 1\n3 0\n\n! 1.5 0.5\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"FKvadratiNaPolyakh"}}}

use std::io::{stdout, Write};
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out, out_line};


fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read();
    let m = input.read();
    if n == 1 && m == 1 {
        println!("! 0 0");
        return;
    }
    let x = if n > 1 {
        find(input, n, m, false)
    } else {
        0.0
    };
    let y = if m > 1 {
        find(input, m, n, true)
    } else {
        0.0
    };
    println!("! {:.15} {:.15}", x, y);
    let _ = stdout().flush();
}

fn find(input: &mut Input, n: i32, m: i32, flip: bool) -> f64 {
    let mut request = Vec::new();
    request.push((-1, 0));
    for i in 0..m {
        request.push((n, i));
        request.push((0, i + 1));
    }
    request.push((-1, m));
    if flip {
        for z in &mut request {
            *z = (z.1, z.0);
        }
    }
    println!("? {}", request.len());
    for (x, y) in request {
        println!("{} {}", x, y);
    }
    let _ = stdout().flush();
    let area = 1.0 - input.read::<f64>();
    (area - 1.0 / (2.0 * n as f64)) / (1.0 - 1.0 / (n as f64)) * (n - 1) as f64
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
