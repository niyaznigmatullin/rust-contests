

pub fn get_all_divisors_for_all_up_to(n: usize) -> Vec<Vec<usize>> {
    let mut d = vec![Vec::new(); n + 1];
    for i in 1..=n {
        for j in (i..=n).step_by(i) {
            d[j].push(i);
        }
    }
    d
}

pub fn euler_function_up_to(n: usize) -> Vec<usize> {
    let mut phi = vec![0; n + 1];
    for i in 1..=n {
        phi[i] = i;
    }
    for i in 2..=n {
        if phi[i] == i {
            for j in (i..=n).step_by(i) {
                phi[j] -= phi[j] / i;
            }
        }
    }
    phi
}