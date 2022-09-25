use std::cmp::Ordering;

pub enum WithInfinity<T> {
    Value(T),
    PlusInfinity,
    MinusInfinity,
}

impl<T> Eq for WithInfinity<T> where T: Ord {}

impl<T> PartialEq<Self> for WithInfinity<T>
    where
        T: Ord,
{
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

impl<T> PartialOrd<Self> for WithInfinity<T>
    where
        T: Ord,
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<T> Ord for WithInfinity<T>
    where
        T: Ord,
{
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (WithInfinity::Value(x), WithInfinity::Value(y)) => x.cmp(y),
            (WithInfinity::PlusInfinity, WithInfinity::PlusInfinity) => Ordering::Equal,
            (WithInfinity::MinusInfinity, WithInfinity::MinusInfinity) => Ordering::Equal,
            (WithInfinity::PlusInfinity, _) | (_, WithInfinity::MinusInfinity) => Ordering::Greater,
            (WithInfinity::MinusInfinity, _) | (_, WithInfinity::PlusInfinity) => Ordering::Less,
        }
    }
}