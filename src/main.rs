use std::panic::catch_unwind;
use crate::COption::*;

#[derive(Debug, PartialEq, Eq, Clone)]
enum COption<T> {
    CNone,
    CSome(T),
}

impl<T> COption<T> {
    pub fn unwrap(self) -> T {
        match self {
            CSome(v) => v,
            CNone => panic!("Cannot unwrap `None`"),
        }
    }
    pub fn is_some(&self) -> bool {
        matches!(*self, CSome(_))
    }
    pub fn is_none(&self) -> bool {
        !self.is_some()
    }
}

fn example(some: bool) -> COption<String> {
    match some {
        true => CSome(String::from("Hello world")),
        false => CNone,
    }
}

fn main() {
    let some: COption<String> = example(true);
    let none: COption<String> = example(false);

    assert!(some.is_some());
    assert!(!some.is_none());
    assert!(!none.is_some());
    assert!(none.is_none());
    assert_eq!("Hello world", some.unwrap());
    assert!(catch_unwind(|| none.unwrap()).is_err());
}
