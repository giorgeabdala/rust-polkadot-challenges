#![allow(dead_code)]
use std::ops::Add;

pub trait Summable {
    type Output;
    fn sum_with(&self, other: &Self) -> Self::Output;
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Store<T> {
    value: T
}

impl<T: Clone + Add<Output = T> + Copy> Store<T> {
    pub fn new(v: T) -> Self {
        Store{value: v}
    }
}

impl <T> Summable for Store<T> where T: Clone + Add<Output = T> + Copy, {
    type Output = T;
    fn sum_with(&self, other: &Self) -> T {
        self.value + other.value
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum_with_u32() {
        let a = Store::new(10u32);
        let b = Store::new(20u32);
        let result = a.sum_with(&b);
        assert_eq!(result, 30u32);
    }

    #[test]
    fn test_sum_with_i64() {
        let a = Store::new(-5i64);
        let b = Store::new(15i64);
        let result = a.sum_with(&b);
        assert_eq!(result, 10i64);
    }
}

fn main() {
    let a = Store::new(30i64);
    let b = Store::new(5i64);
    println!("{}", a.sum_with(&b));
} 