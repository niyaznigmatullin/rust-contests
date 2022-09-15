//{"name":"D. Вертикальные пути","group":"Codeforces - Codeforces Round #787 (Div. 3)","url":"https://codeforces.com/contest/1675/problem/D","interactive":false,"timeLimit":2000,"tests":[{"input":"6\n5\n3 1 3 3 1\n4\n1 1 4 1\n7\n1 1 2 3 4 5 6\n1\n1\n6\n4 4 4 4 1 2\n4\n2 2 2 2\n","output":"3\n3\n3 1 5\n1\n2\n1\n4\n\n2\n2\n1 2\n2\n4 3\n\n1\n7\n1 2 3 4 5 6 7\n\n1\n1\n1\n\n3\n3\n4 1 5\n2\n2 6\n1\n3\n\n3\n2\n2 1\n1\n3\n1\n4\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DVertikalniePuti"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read();
    let mut p = input.read_vec(n);
    for x in &mut p {
        *x -= 1;
    }
    let mut deg = vec![0; n];
    for i in 0..n {
        if p[i] != i {
            deg[p[i]] += 1;
        }
    }
    let mut used = vec![false; n];
    let mut answer = Vec::new();
    for mut i in 0..n {
        if deg[i] > 0 {
            continue;
        }
        let mut path = Vec::new();
        while !used[i] {
            path.push(i);
            used[i] = true;
            i = p[i];
        }
        path.reverse();
        answer.push(path);
    }
    out_line!(answer.len());
    for x in answer {
        out_line!(x.len());
        out_line!(x.iter().map(|y| (y + 1).to_string()).collect::<Vec<_>>().join(" "));
    }
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
