//{"name":"j_trains","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"j_trains"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out, out_line};
use std::cmp::max;
use std::collections::{BTreeSet, HashMap};

fn solve(input: &mut Input, _test_case: usize) {
    let (n, _, m): (usize, usize, i32) = input.read();
    let mut a = input
        .read_vec::<String>(n)
        .into_iter()
        .map(|x| x.into_bytes())
        .collect::<Vec<_>>();
    let mut table = HashMap::<Vec<u8>, BTreeSet<usize>>::new();
    let mut ans = vec![0; n];
    for i in 0..n {
        add_to_table_and_relax(&mut table, &mut ans, i, a[i].clone());
    }
    for _ in 0..m {
        let p1 = input.read::<usize>() - 1;
        let w1 = input.read::<usize>() - 1;
        let p2 = input.read::<usize>() - 1;
        let w2 = input.read::<usize>() - 1;
        remove_from_table(&mut table, p1, &a[p1]);
        if p1 != p2 {
            remove_from_table(&mut table, p2, &a[p2]);
        }
        {
            let t = a[p1][w1];
            a[p1][w1] = a[p2][w2];
            a[p2][w2] = t;
        }
        add_to_table_and_relax(&mut table, &mut ans, p1, a[p1].clone());
        if p1 != p2 {
            add_to_table_and_relax(&mut table, &mut ans, p2, a[p2].clone());
        }
    }
    for x in ans {
        out_line!(x);
    }
}

fn add_to_table_and_relax(
    table: &mut HashMap<Vec<u8>, BTreeSet<usize>>,
    ans: &mut Vec<usize>,
    id: usize,
    key: Vec<u8>,
) {
    let z = table.entry(key).or_default();
    z.insert(id);
    let size = z.len();
    for &x in z.iter() {
        ans[x] = max(ans[x], size);
    }
}

fn remove_from_table(table: &mut HashMap<Vec<u8>, BTreeSet<usize>>, id: usize, key: &Vec<u8>) {
    let z = table.get_mut(key).unwrap();
    z.remove(&id);
    if z.is_empty() {
        table.remove(key);
    }
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
