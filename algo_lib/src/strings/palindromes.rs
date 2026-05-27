use std::cmp::min;

pub fn find_palindromic_radius<T: PartialEq>(s: &[T]) -> Vec<usize> {
    let mut d = vec![0; s.len()];
    let mut best = 0;
    for i in 0..s.len() {
        if best + d[best] > i {
            d[i] = min(d[2 * best - i], best + d[best] - i)
        }
        while i >= d[i] && i + d[i] < s.len() && s[i - d[i]] == s[i + d[i]] {
            d[i] += 1;
        }
        if i + d[i] > best + d[best] {
            best = i
        }
    }
    d
}
