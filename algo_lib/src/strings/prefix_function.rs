

pub fn prefix_function<T>(s : &Vec<T>) -> Vec<usize>
    where T : Eq {
    let mut p = vec![0; s.len()];
    let mut k = 0;
    for i in 1..s.len() {
        while k > 0 && s[i] != s[k] {
            k = p[k - 1];
        }
        if s[i] == s[k] {
            k += 1;
        }
        p[i] = k;
    }
    p
}
