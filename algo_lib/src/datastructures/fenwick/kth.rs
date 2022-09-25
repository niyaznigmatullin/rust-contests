pub struct FenwickKth {
    a: Vec<usize>,
}

impl FenwickKth {
    pub fn new(size: usize) -> Self {
        let mut n = 1;
        while n < size {
            n *= 2;
        }
        Self { a: vec![0; n] }
    }

    fn change(&mut self, mut i: usize, f: impl Fn(&mut usize)) {
        while i < self.a.len() {
            f(&mut self.a[i]);
            i |= i + 1;
        }
    }

    pub fn add(&mut self, i: usize, delta: usize) {
        self.change(i, |x| *x += delta);
    }

    pub fn remove(&mut self, i: usize, delta: usize) {
        self.change(i, |x| *x -= delta);
    }

    pub fn get_kth_pos(&self, mut k: usize) -> Option<usize> {
        let mut left = 0;
        let mut right = self.a.len();
        if self.a[right - 1] < k || k == 0 {
            return None;
        }
        while left + 1 < right {
            let mid = (left + right) >> 1;
            if self.a[mid - 1] >= k {
                right = mid;
            } else {
                k -= self.a[mid - 1];
                left = mid;
            }
        }
        Some(left)
    }
}
