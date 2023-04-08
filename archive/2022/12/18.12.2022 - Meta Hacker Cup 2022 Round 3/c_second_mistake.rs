//{"name":"C: Second Mistake","group":"Meta Coding Competitions - Meta Hacker Cup 2022 Round 3","url":"https://www.facebook.com/codingcompetitions/hacker-cup/2022/round-3/problems/C","interactive":false,"timeLimit":360000,"tests":[{"input":"3\n1\nmetamate\n5\nteammate\nmeatmate\nmetatame\nmememate\nmetameme\n3\nmeet\nemma\ntate\n2\ntata\nmaam\n3\nmem\nmet\nmat\n3\ntam\nmat\ntea\n","output":"Case #1: 4\nCase #2: 0\nCase #3: 5\n"}],"testType":"multiNumber","input":{"type":"regex","fileName":null,"pattern":"second_mistake_.*input[.]txt"},"output":{"type":"file","fileName":"second_mistake_output.txt","pattern":null},"languages":{"java":{"taskClass":"CSecondMistake"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::math::modular::primitive::{Modular, ModularType};
use algo_lib::{out, out_line};
use std::collections::HashMap;

struct Hasher<const M: u32> {
    pow: Vec<ModularType<M>>,
}

impl<const M: u32> Hasher<M> {
    fn new(n: usize, x: u32) -> Self {
        let x = ModularType::<M>::from(x);
        let mut pow = vec![ModularType::<M>::from(1); n + 1];
        for i in 1..=n {
            pow[i] = pow[i - 1] * x;
        }
        Self { pow }
    }

    fn hash(&self, s: &Vec<u8>) -> ModularType<M> {
        let mut hash = ModularType::<M>::from(0);
        let x = self.pow[1];
        for &c in s {
            hash = hash * x + ModularType::<M>::from(c as u32);
        }
        hash
    }

    fn rehash(&self, mut hash: ModularType<M>, index: usize, from: u8, to: u8) -> ModularType<M> {
        hash -= self.pow[index] * ModularType::<M>::from(from as u32);
        hash += self.pow[index] * ModularType::<M>::from(to as u32);
        hash
    }
}

const M1: u32 = 1000000007;
const M2: u32 = 998244353;
struct AllHasher {
    h1: Hasher<M1>,
    h2: Hasher<M2>,
}

type MyHash = (u32, u32);

impl AllHasher {
    fn new(n: usize) -> Self {
        let h1 = Hasher::<M1>::new(n, 239017);
        let h2 = Hasher::<M2>::new(n, 33533);
        Self { h1, h2 }
    }

    fn hash(&self, s: &Vec<u8>) -> MyHash {
        let hash1 = self.h1.hash(s);
        let hash2 = self.h2.hash(s);
        (hash1.value(), hash2.value())
    }

    fn rehash(&self, hash: MyHash, index: usize, from: u8, to: u8) -> MyHash {
        (
            self.h1
                .rehash(ModularType::<M1>::from(hash.0), index, from, to)
                .value(),
            self.h2
                .rehash(ModularType::<M2>::from(hash.1), index, from, to)
                .value(),
        )
    }
}

fn solve(input: &mut Input, _test_case: usize, hasher: &AllHasher) {
    eprintln!("Running {}", _test_case);
    let n = input.read();
    let alphabet = [b'm', b'e', b't', b'a'];
    let mut table = HashMap::new();
    let mut table_no_change = HashMap::new();
    for _ in 0..n {
        let s = input.read::<String>().into_bytes();
        let hash = hasher.hash(&s);
        *table_no_change.entry(hash).or_insert(0) += 1;
        for pos in 0..s.len() {
            for letter in alphabet {
                if letter == s[pos] {
                    continue;
                }
                let new_hash = hasher.rehash(hash, s.len() - pos - 1, s[pos], letter);
                *table.entry(new_hash).or_insert(0) += 1;
                // eprintln!("{:?} {:?}", new_hash, (&s, pos, s[pos], letter));
            }
        }
    }
    let q = input.read();
    let mut sum = 0i64;
    for _ in 0..q {
        let s = input.read::<String>().into_bytes();
        let hash = hasher.hash(&s);
        let mut ans = 0;
        for pos in 0..s.len() {
            for letter in alphabet {
                if letter == s[pos] {
                    continue;
                }
                let new_hash = hasher.rehash(hash, s.len() - pos - 1, s[pos], letter);
                let got = *table.get(&new_hash).unwrap_or(&0);
                // eprintln!("   {:?} {:?} {}", new_hash, (&s, pos, s[pos], letter), got);
                ans += got;
            }
        }
        let one_diff = *table.get(&hash).unwrap_or(&0);
        // eprintln!("ans = {}", ans);
        ans -= 2 * one_diff;
        ans -= (*table_no_change.get(&hash).unwrap_or(&0) as i64) * 3i64 * s.len() as i64;
        ans /= 2;
        // eprintln!("ans = {}", ans);
        sum += ans as i64;
    }
    out_line!(format!("Case #{}: {}", _test_case, sum));
}

pub(crate) fn run(mut input: Input) -> bool {
    let t = input.read();
    let hasher = AllHasher::new(20000);
    for i in 0usize..t {
        solve(&mut input, i + 1, &hasher);
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
