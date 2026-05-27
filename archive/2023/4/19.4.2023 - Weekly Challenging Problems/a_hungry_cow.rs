//{"name":"A. Hungry Cow","group":"Codeforces - Weekly Challenging Problems","url":"https://codeforces.com/group/7CDWDjbWHM/contest/438003/problem/A","interactive":false,"timeLimit":6000,"tests":[{"input":"3\n4 3\n1 5\n1 2\n","output":"15\n36\n18\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"AHungryCow"}}}

use algo_lib::datastructures::segment_tree::common::SegmentTreeNode;
use algo_lib::datastructures::segment_tree::persistent::simple::PSegTreeNode;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::math::modular::primitive::ModularType;
use algo_lib::{out, out_line};
use std::cmp::{max, min};
use std::collections::{BTreeSet, HashMap};
use std::rc::Rc;

enum Modification {
    ADD(Segment),
    REMOVE(Segment),
}

struct IncrementalDS {
    segments: BTreeSet<Segment>,
    sum: Mod,
    modifications: Vec<Modification>,
}

type Mod = ModularType<1000000007>;

fn sum_up_to(x: i64) -> Mod {
    if (x & 1) == 1 {
        Mod::from((x + 1) / 2) * Mod::from(x)
    } else {
        Mod::from(x / 2) * Mod::from(x + 1)
    }
}

#[derive(Ord, PartialOrd, Eq, PartialEq, Clone)]
struct Segment {
    start: i64,
    end: i64,
}

impl Segment {
    fn new(start: i64, end: i64) -> Self {
        Self { start, end }
    }

    fn sum(&self) -> Mod {
        sum_up_to(self.end - 1) - sum_up_to(self.start - 1)
    }
}

impl IncrementalDS {
    fn new() -> Self {
        Self {
            segments: BTreeSet::new(),
            sum: Mod::from(0),
            modifications: Vec::new(),
        }
    }

    fn add_internal(&mut self, range: Segment) {
        self.modifications.push(Modification::ADD(range.clone()));
        self.do_add_internal(range);
    }

    fn do_add_internal(&mut self, range: Segment) {
        self.sum += range.sum();
        self.segments.insert(range);
    }

    fn remove_internal(&mut self, range: &Segment) {
        self.modifications.push(Modification::REMOVE(range.clone()));
        self.do_remove_internal(range);
    }

    fn do_remove_internal(&mut self, range: &Segment) {
        self.segments.remove(range);
        self.sum -= range.sum();
    }

    fn add(&mut self, day: i64, mut bales: i64) {
        let s = Segment::new(day, day);
        if let Some(last) = self.segments.range(..&s).next_back() {
            if last.end >= day {
                self.add(last.start, bales);
                return;
            }
        }
        while let Some(next) = self.segments.range(&s..).next() {
            if next.start <= day + bales {
                bales += next.end - next.start;
                let copied = next.clone();
                self.remove_internal(&copied);
            } else {
                break;
            }
        }
        self.add_internal(Segment::new(day, day + bales));
    }

    fn query(&self) -> Mod {
        self.sum
    }

    fn snapshot(&self) -> usize {
        self.modifications.len()
    }

    fn restore(&mut self, snap: usize) {
        while self.modifications.len() > snap {
            match self.modifications.pop().unwrap() {
                Modification::ADD(s) => {
                    self.do_remove_internal(&s);
                }
                Modification::REMOVE(s) => {
                    self.do_add_internal(s);
                }
            }
        }
    }
}

#[derive(Clone)]
struct Node {
    sum: Mod,
    from: i64,
    to: i64,
    prefix: i64,
    count: i64,
}

impl SegmentTreeNode for Node {
    fn neutral() -> Self {
        todo!()
    }

    fn join(left: &Self, right: &Self) -> Self {
        Self {
            sum: left.sum + right.sum,
            from: left.from,
            to: right.to,
            prefix: if left.prefix + left.from == left.to {
                left.prefix + right.prefix
            } else {
                left.prefix
            },
            count: left.count + right.count,
        }
    }
}

#[derive(Clone)]
struct STree {
    root: Rc<PSegTreeNode<Node>>,
}

impl STree {
    fn new(n: usize, a: &[i64]) -> Self {
        Self {
            root: Rc::new(Self::build(0, n, a)),
        }
    }

    fn build(left: usize, right: usize, a: &[i64]) -> PSegTreeNode<Node> {
        if left + 1 == right {
            return PSegTreeNode::new_leaf(Node {
                sum: Mod::from(0),
                from: a[left],
                to: a[left + 1],
                prefix: 0,
                count: 0,
            });
        }
        let mid = (left + right) >> 1;
        let left_child = Self::build(left, mid, a);
        let right_child = Self::build(mid, right, a);
        PSegTreeNode::new_from_children(Rc::new(left_child), Rc::new(right_child))
    }

    fn add(&self, day: i64, value: i64) -> Self {
        let (new_root, added) = Self::add_internal(&self.root, day, value);
        assert_eq!(added, value);
        STree { root: new_root }
    }

    fn add_internal(
        node: &Rc<PSegTreeNode<Node>>,
        mut day: i64,
        value: i64,
    ) -> (Rc<PSegTreeNode<Node>>, i64) {
        let v = &node.value;
        if value == 0 || v.to <= day {
            return (node.clone(), 0);
        }
        day = max(day, v.from);
        if (day <= v.from + v.prefix && v.count + value + v.from >= v.to) || node.left.is_none() {
            let add = min(v.to - v.from - v.count, value);
            let prefix = v.count + add;
            let sum = sum_up_to(v.from + prefix - 1) - sum_up_to(v.from - 1);
            let new_v = Node {
                prefix,
                count: prefix,
                sum,
                ..*v
            };
            return (Rc::new(PSegTreeNode::new_leaf(new_v)), add);
        }
        let (new_left, left_added) = Self::add_internal(node.get_left(), day, value);
        let (new_right, right_added) =
            Self::add_internal(node.get_right(), day, value - left_added);
        let new_node = PSegTreeNode::new_from_children(new_left, new_right);
        (Rc::new(new_node), left_added + right_added)
    }

    fn query(&self) -> Mod {
        self.root.value.sum
    }
}

struct Query {
    day: i64,
    bales_before: i64,
    bales: i64,
    prev: usize,
    next: usize,
    answer: Mod,
}

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read();
    // let mut ds = IncrementalDS::new();
    let mut table: HashMap<i64, usize> = HashMap::new();
    let mut queries: Vec<Query> = input
        .read_vec::<(i64, i64)>(n)
        .into_iter()
        .map(|(day, bales)| Query {
            day,
            bales_before: 0,
            bales,
            prev: n,
            next: n,
            answer: Mod::from(0),
        })
        .collect::<Vec<_>>();
    let mut all = Vec::new();
    for i in 0..n {
        let day = queries[i].day;
        if let Some(&y) = table.get(&day) {
            queries[y].next = i - y;
            queries[i].prev = i - y;
            queries[i].bales_before = queries[y].bales;
        }
        all.push(day);
        table.insert(day, i);
    }
    all.push(i64::MAX / 2);
    all.sort();
    all.dedup();
    let ds = STree::new(all.len() - 1, &all);
    go(&mut queries[..], ds);
    for x in queries {
        out_line!(x.answer);
    }
}

fn go(qs: &mut [Query], ds: STree) {
    if qs.len() == 1 {
        let q = &mut qs[0];
        let new_ds = ds.add(q.day, q.bales);
        q.answer = new_ds.query();
        return;
    }
    let mid = qs.len() / 2;
    let mut new_ds = ds.clone();
    for i in mid..qs.len() {
        let q = &qs[i];
        if i < q.prev {
            new_ds = new_ds.add(q.day, q.bales_before);
        }
    }
    go(&mut qs[..mid], new_ds);
    new_ds = ds.clone();
    for i in 0..mid {
        let q = &qs[i];
        if i + q.next >= qs.len() {
            new_ds = new_ds.add(q.day, q.bales);
        }
    }
    go(&mut qs[mid..], new_ds);
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
