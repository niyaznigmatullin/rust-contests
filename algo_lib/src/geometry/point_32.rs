use crate::io::input::{Input, Readable};
use std::ops::{Add, Sub};

#[derive(Clone, Copy, Eq, PartialEq)]
pub struct Point32 {
    pub x: i32,
    pub y: i32,
}

impl Point32 {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

impl Point32 {
    pub fn vmul(&self, other: &Self) -> i64 {
        self.x as i64 * other.y as i64 - self.y as i64 * other.x as i64
    }

    pub fn smul(&self, other: &Self) -> i64 {
        self.x as i64 * other.x as i64 + self.y as i64 * other.y as i64
    }

    pub fn len_squared(&self) -> i64 {
        self.smul(self)
    }
}

impl Sub for Point32 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl Add for Point32 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Readable for Point32 {
    fn read(input: &mut Input) -> Self {
        let (x, y) = input.read();
        Self { x, y }
    }
}
