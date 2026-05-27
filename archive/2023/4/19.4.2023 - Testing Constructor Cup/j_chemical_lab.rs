//{"name":"J. Chemical Lab","group":"Codeforces - Testing Constructor Cup","url":"https://codeforces.com/gym/435951/problem/J","interactive":false,"timeLimit":2000,"tests":[{"input":"3 3\n1 2 3\n4 5 6\n","output":"4\n"},{"input":"2 3\n3 4\n1 6 8\n","output":"-1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"JChemicalLab"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out, out_line};
use std::cmp::min;
use std::collections::VecDeque;

struct Node {
    sum: usize,
    id: usize,
}

impl Node {
    fn new(value: usize, id: usize) -> Self {
        Self { sum: value, id }
    }
}

struct Graph {
    nodes: Vec<Node>,
    edges: Vec<Vec<usize>>,
    result: Vec<usize>,
}

impl Graph {
    fn new() -> Self {
        Self {
            nodes: vec![Node::new(0, 0)],
            edges: vec![Vec::new()],
            result: Vec::new(),
        }
    }

    fn init(&mut self) {
        self.result = vec![0; self.nodes.len()];
    }

    fn add_leaf(&mut self, v: usize, delta: usize) -> usize {
        let new_v = self.nodes.len();
        self.nodes.push(Node::new(self.nodes[v].sum + delta, new_v));
        self.edges.push(Vec::new());
        self.edges[v].push(new_v);
        self.edges[new_v].push(v);
        new_v
    }

    fn keep_only(&mut self, value: usize) {
        (0..self.nodes.len())
            .filter(|&x| self.nodes[x].sum != value)
            .for_each(|x| self.result[x] = usize::MAX);
    }

    fn get_best(&self) -> usize {
        *self.result.iter().min().unwrap()
    }

    fn bfs(&mut self) {
        let mut q = VecDeque::new();
        let mut in_queue = vec![false; self.nodes.len()];
        for v in (0..self.nodes.len()).filter(|&x| self.result[x] != usize::MAX) {
            in_queue[v] = true;
            q.push_back(v);
        }
        while let Some(v) = q.pop_front() {
            in_queue[v] = false;
            let cur_path = self.result[v];
            for &to in self.edges[v].iter() {
                if self.result[to] > cur_path + 1 {
                    self.result[to] = cur_path + 1;
                    if !in_queue[to] {
                        in_queue[to] = true;
                        q.push_back(to);
                    }
                }
            }
        }
    }
}

struct PartitionGenerator {
    allowed: Vec<usize>,
    graph: Graph,
}

impl PartitionGenerator {
    fn new(allowed: Vec<usize>) -> Self {
        Self {
            allowed,
            graph: Graph::new(),
        }
    }

    fn gen(&mut self, n: usize, k: usize, v: usize) {
        let up_to = min(n, k);
        for index in 0..self.allowed.len() {
            let x = self.allowed[index];
            if x > up_to {
                break;
            }
            let u = self.graph.add_leaf(v, x);
            self.gen(n - x, x, u);
        }
    }

    fn generate(mut self, n: usize) -> Graph {
        self.gen(n, n, 0);
        self.graph.init();
        self.graph
    }
}

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read();
    let m: usize = input.read();
    let allowed = input.read_vec(n);
    let mut graph = PartitionGenerator::new(allowed).generate(40);
    graph.keep_only(0);
    for _ in 0..m {
        let r = input.read();
        graph.bfs();
        graph.keep_only(r);
    }
    let ans = match graph.get_best() {
        usize::MAX => -1,
        x => x as i64,
    };
    out_line!(ans);
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
