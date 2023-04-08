//{"name":"B. Поворот матрицы","group":"Codeforces - Codeforces Round #839 (Div. 3)","url":"https://codeforces.com/contest/1772/problem/B","interactive":false,"timeLimit":2000,"tests":[{"input":"6\n1 3\n5 7\n8 10\n3 4\n8 10\n4 3\n6 1\n9 2\n7 5\n4 2\n1 2\n4 3\n","output":"YES\nYES\nNO\nYES\nYES\nNO\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BPovorotMatritsi"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let mut a = input.read_vec::<i32>(4);
    a.swap(2, 3);
    for i in 0..4 {
        if !(a[0] >= a[1] || a[0] >= a[3] || a[1] >= a[2] || a[3] >= a[2]) {
            out_line!("YES");
            return;
        }
        a.rotate_left(1);
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
