//{"name":"e_byties_boy_walk","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"e_byties_boy_walk"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out, out_line};
use std::collections::{BTreeSet, HashSet, VecDeque};

#[derive(Clone)]
struct Value {
    length: usize,
    letter: usize,
    first: usize,
    last: usize,
}

fn solve(input: &mut Input, _test_case: usize) {
    let (n, m) = input.read();
    let mut edges = vec![vec![Vec::new(); 26]; n];
    let mut rev_edges = vec![vec![Vec::new(); 26]; n];
    for _ in 0..m {
        let from = input.read::<usize>() - 1;
        let to = input.read::<usize>() - 1;
        let color = (input.read::<String>().into_bytes()[0] - b'a') as usize;
        edges[from][color].push(to);
        rev_edges[to][color].push(from);
    }
    let mut ans = vec![
        vec![
            Value {
                length: usize::MAX,
                letter: 0,
                first: 0,
                last: 0
            };
            n
        ];
        n
    ];
    let mut q = VecDeque::new();
    for i in 0..n {
        q.push_back((i, i));
        ans[i][i] = Value {
            length: 0,
            letter: 0,
            first: 0,
            last: 0,
        };
    }
    for i in 0..n {
        for color in 0..26 {
            for &j in &edges[i][color] {
                ans[i][j] = Value {
                    length: 1,
                    letter: color,
                    first: j,
                    last: 0,
                };
                q.push_back((i, j));
            }
        }
    }
    while let Some((from, to)) = q.pop_front() {
        for color in 0..26 {
            let r1 = &rev_edges[from];
            let r2 = &edges[to];
            if r1.is_empty() || r2.is_empty() {
                continue;
            }
            for &new_from in &r1[color] {
                for &new_to in &r2[color] {
                    if ans[new_from][new_to].length == usize::MAX {
                        ans[new_from][new_to] = Value {
                            length: ans[from][to].length + 2,
                            letter: color,
                            first: from,
                            last: to,
                        };
                        q.push_back((new_from, new_to));
                    }
                }
            }
        }
    }
    let q = input.read();
    let mut last = input.read::<usize>() - 1;
    for _ in 1..q {
        let next = input.read::<usize>() - 1;
        let cur_answer = if ans[last][next].length == usize::MAX {
            (usize::MAX, "".to_string())
        } else {
            let mut from = last;
            let mut to = next;
            let mut str = String::new();
            while ans[from][to].length > 1 {
                let value = &ans[from][to];
                add_color(&mut str, value.letter);
                from = value.first;
                to = value.last;
            }
            let str_rev = str.chars().rev().collect::<String>();
            if ans[from][to].length == 1 {
                add_color(&mut str, ans[from][to].letter);
            }
            str.push_str(&str_rev);
            (str.len(), str)
        };
        if cur_answer.0 == usize::MAX {
            out_line!("-1");
        } else {
            out_line!(cur_answer.0, cur_answer.1);
        }
        last = next;
    }
}

fn add_color(str: &mut String, color: usize) {
    str.push(char::from(color as u8 + b'a'));
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
