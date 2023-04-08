//{"name":"B: Third Trie","group":"Meta Coding Competitions - Meta Hacker Cup 2022 Round 3","url":"https://www.facebook.com/codingcompetitions/hacker-cup/2022/round-3/problems/B","interactive":false,"timeLimit":360000,"tests":[{"input":"3\n3\n3\n1 a\n1 b\n3\n1 a\n2 a\n2\n1 c\n4\n2\n1 a\n2\n1 a\n2\n1 a\n4\n1 a\n2 a\n3 a\n4\n2\n1 a\n2\n1 b\n2\n1 c\n4\n1 a\n2 b\n3 c\n","output":"Case #1: 5\nCase #2: 14\nCase #3: 20\n"}],"testType":"multiNumber","input":{"type":"regex","fileName":null,"pattern":"third_trie_.*input[.]txt"},"output":{"type":"file","fileName":"third_trie_output.txt","pattern":null},"languages":{"java":{"taskClass":"BThirdTrie"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out, out_line};
use std::collections::HashMap;

#[derive(Debug)]
struct Node {
    children: HashMap<u8, usize>,
    count: i64,
}

impl Node {
    fn new() -> Self {
        Self {
            children: HashMap::new(),
            count: 0,
        }
    }
}

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read::<usize>();
    let mut trie = Vec::new();
    trie.push(Node::new());
    for _ in 0..n {
        let m = input.read::<usize>();
        let mut in_trie = Vec::<usize>::new();
        in_trie.push(0);
        trie[0].count += 1;
        for _ in 1..m {
            let parent = in_trie[input.read::<usize>() - 1];
            let c = input.read::<String>().into_bytes()[0];
            let current = if let Some(&current) = trie[parent].children.get(&c) {
                current
            } else {
                let id = trie.len();
                trie.push(Node::new());
                trie[parent].children.insert(c, id);
                id
            };
            trie[current].count += 1;
            in_trie.push(current);
        }
    }
    let mut ans = 0;
    for v in 0..trie.len() {
        let count = trie[v].count;
        {
            ans += (n - 1) as i64 * (n - 2) as i64 / 2 * count;
        }
        {
            let count = count * (count - 1) / 2;
            ans -= (n - 2) as i64 * count;
        }
        {
            let count = count * (count - 1) * (count - 2) / 6;
            ans += count;
        }
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
