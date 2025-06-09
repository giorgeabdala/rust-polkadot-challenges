use std::fmt::Display;

pub trait Description {
    fn describe(&self) -> String;
}

struct Store<T> {
    value: T,
}

struct Describable<T>(pub T);

impl<T> Store<T> {
    pub fn new(value: T) -> Self {
        Store { value }
    }
}

impl<T> Describable<T> {
    pub fn new(value: T) -> Self {
        Describable(value)
    }
}

impl Description for Store<u32> {
    fn describe(&self) -> String {
        format!("Numeric value: {}", self.value)
    }
}

impl Description for Store<&'static str> {
    fn describe(&self) -> String {
        format!("{} Hello, Rust", self.value)
    }
}

impl<T: Display> Description for Describable<T> {
    fn describe(&self) -> String {
        format!("Content: {}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn describe_u32() {
        let s = Store::new(100u32);
        assert_eq!(s.describe(), "Numeric value: 100");
    }

    #[test]
    fn describe_str() {
        let s = Store::new("Rust is Awesome!");
        assert_eq!(s.describe(), "Rust is Awesome! Hello, Rust");
    }

    #[test]
    fn describe_generic() {
        let s = Describable::new(true);
        assert_eq!(s.describe(), "Content: true");
    }
}

fn main() {}
