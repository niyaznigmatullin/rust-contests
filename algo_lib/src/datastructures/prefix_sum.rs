use std::ops::{AddAssign, Range, Sub};

pub struct StaticRSQ<T> {
    prefix_sum: Vec<T>,
}

impl<T> StaticRSQ<T>
where
    T: Copy + AddAssign + Sub<Output = T>,
{
    pub fn new(mut prefix_sum: Vec<T>) -> Self {
        if !prefix_sum.is_empty() {
            let mut sum = prefix_sum[0];
            for x in prefix_sum.iter_mut().skip(1) {
                sum += *x;
                *x = sum;
            }
        }
        Self { prefix_sum }
    }

    pub fn new_from<S>(a: &Vec<S>) -> Self
    where
        for<'a> T: From<&'a S>,
    {
        let a = a.iter().map(|x| x.into()).collect::<Vec<T>>();
        Self::new(a)
    }

    pub fn get_sum(&self, range: Range<usize>) -> T {
        if range.start == 0 {
            self.prefix_sum[range.end - 1]
        } else {
            self.prefix_sum[range.end - 1] - self.prefix_sum[range.start - 1]
        }
    }

    pub fn len(&self) -> usize {
        self.prefix_sum.len()
    }
}
