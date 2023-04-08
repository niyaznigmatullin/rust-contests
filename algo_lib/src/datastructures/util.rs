use std::cmp::Ordering;
use std::cmp::Ordering::Greater;
use std::iter::Peekable;
use std::ops::Sub;

pub struct MergeSorted<T, A: Iterator<Item = T>, B: Iterator<Item = T>, F> {
    a: Peekable<A>,
    b: Peekable<B>,
    compare: F,
}

impl<T, A, B, F> Iterator for MergeSorted<T, A, B, F>
where
    A: Iterator<Item = T>,
    B: Iterator<Item = T>,
    F: FnMut(&T, &T) -> Ordering,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        match (self.a.peek(), self.b.peek()) {
            (None, None) => None,
            (None, _) => self.b.next(),
            (_, None) => self.a.next(),
            (Some(a), Some(b)) if (self.compare)(a, b) == Greater => self.b.next(),
            _ => self.a.next(),
        }
    }
}

pub fn merge<T: Ord, A, B>(a: A, b: B) -> MergeSorted<T, A, B, impl FnMut(&T, &T) -> Ordering>
where
    A: Iterator<Item = T>,
    B: Iterator<Item = T>,
{
    MergeSorted {
        a: a.peekable(),
        b: b.peekable(),
        compare: |x: &T, y: &T| x.cmp(y),
    }
}

pub fn merge_by<T, A, B, F>(a: A, b: B, compare: F) -> MergeSorted<T, A, B, F>
where
    A: Iterator<Item = T>,
    B: Iterator<Item = T>,
    F: FnMut(&T, &T) -> Ordering,
{
    MergeSorted {
        a: a.peekable(),
        b: b.peekable(),
        compare,
    }
}

pub fn merge_by_key<T, A, B, F, K>(
    a: A,
    b: B,
    mut f: F,
) -> MergeSorted<T, A, B, impl FnMut(&T, &T) -> Ordering>
where
    A: Iterator<Item = T>,
    B: Iterator<Item = T>,
    F: FnMut(&T) -> K,
    K: Ord,
{
    MergeSorted {
        a: a.peekable(),
        b: b.peekable(),
        compare: move |x: &T, y: &T| f(x).cmp(&f(y)),
    }
}

pub fn concatenate<I1, I2, E>(a: I1, b: I2) -> Vec<E>
where
    I1: IntoIterator<Item = E>,
    I2: IntoIterator<Item = E>,
{
    a.into_iter().chain(b.into_iter()).collect::<Vec<_>>()
}

pub fn subtract_one<I, T>(a: I) -> Vec<T>
where
    I: IntoIterator<Item = T>,
    T: Sub<T, Output = T> + From<u8>,
{
    a.into_iter().map(|x| x - 1.into()).collect()
}
