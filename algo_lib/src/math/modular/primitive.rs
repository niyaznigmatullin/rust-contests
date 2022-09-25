use crate::io::output::{Output, Writable};
use std::ops::{Add, AddAssign, Mul, MulAssign, Neg, Sub, SubAssign};

#[derive(Copy, Clone)]
pub struct ModularType<const M: u32>(u32);

pub trait Modular:
    Add<Output = Self>
    + AddAssign
    + Mul<Output = Self>
    + MulAssign
    + Sub<Output = Self>
    + SubAssign
    + Neg<Output = Self>
    + From<usize>
    + From<i32>
    + From<u32>
    + From<u64>
    + Writable
    + Copy
{
    const MODULO: u32;
    fn value(&self) -> u32;
    fn usize(&self) -> usize {
        self.value() as usize
    }
    fn power(self, mut d: usize) -> Self {
        let mut result = Self::from(1);
        let mut a = self;
        while d > 0 {
            if (d & 1) == 1 {
                result *= a;
            }
            a *= a;
            d >>= 1;
        }
        result
    }
}

impl<const M: u32> Modular for ModularType<M> {
    const MODULO: u32 = M;

    fn value(&self) -> u32 {
        self.0
    }
}

impl<const M: u32> Add for ModularType<M> {
    type Output = ModularType<M>;

    fn add(self, rhs: Self) -> Self::Output {
        ModularType(add::<M>(self.0, rhs.0))
    }
}

fn add<const M: u32>(a: u32, b: u32) -> u32 {
    let result = a + b;
    if result >= M {
        result - M
    } else {
        result
    }
}

impl<const M: u32> AddAssign for ModularType<M> {
    fn add_assign(&mut self, rhs: Self) {
        self.0 = add::<M>(self.0, rhs.0);
    }
}

impl<const M: u32> Sub for ModularType<M> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(add::<M>(self.0, M - rhs.0))
    }
}

impl<const M: u32> SubAssign for ModularType<M> {
    fn sub_assign(&mut self, rhs: Self) {
        self.0 = add::<M>(self.0, M - rhs.0);
    }
}

impl<const M: u32> Mul for ModularType<M> {
    type Output = ModularType<M>;

    fn mul(self, rhs: Self) -> Self::Output {
        Self(multiply::<M>(self.0, rhs.0))
    }
}

impl<const M: u32> MulAssign for ModularType<M> {
    fn mul_assign(&mut self, rhs: Self) {
        self.0 = multiply::<M>(self.0, rhs.0);
    }
}

fn multiply<const M: u32>(a: u32, b: u32) -> u32 {
    (a as u64 * b as u64 % M as u64) as u32
}

impl<const M: u32> Neg for ModularType<M> {
    type Output = ModularType<M>;

    fn neg(self) -> Self::Output {
        Self(if self.0 == 0 { 0 } else { M - self.0 })
    }
}

impl<const M: u32> From<u32> for ModularType<M> {
    fn from(x: u32) -> Self {
        Self(x % M)
    }
}

impl<const M: u32> From<i32> for ModularType<M> {
    fn from(x: i32) -> Self {
        Self(x.rem_euclid(M as i32) as u32)
    }
}

impl<const M: u32> From<usize> for ModularType<M> {
    fn from(x: usize) -> Self {
        Self((x % M as usize) as u32)
    }
}

impl<const M: u32> Into<usize> for ModularType<M> {
    fn into(self) -> usize {
        self.0 as usize
    }
}

impl<const M: u32> Writable for ModularType<M> {
    fn write(&self, output: &mut Output) {
        self.0.write(output);
    }
}

impl<const M: u32> From<u64> for ModularType<M> {
    fn from(x: u64) -> Self {
        Self((x % M as u64) as u32)
    }
}

impl<const M: u32> From<i64> for ModularType<M> {
    fn from(x: i64) -> Self {
        Self(x.rem_euclid(M as i64) as u32)
    }
}
