use std::panic::catch_unwind;
use crate::CustomOption::*;

#[derive(Debug, PartialEq, Eq, Clone)]
enum CustomOption<T> {
    CustomNone,
    CustomSome(T),
}

impl<T> CustomOption<T> {
    pub fn unwrap(self) -> T {
        match self {
            CustomSome(v) => v,
            CustomNone => panic!("Cannot unwrap `OptNone`"),
        }
    }
    pub fn is_some(&self) -> bool {
        matches!(*self, CustomSome(_))
    }
    pub fn is_none(&self) -> bool {
        !self.is_some()
    }
}

fn example(some: bool) -> CustomOption<String> {
    match some {
        true => CustomSome(String::from("Hello world")),
        false => CustomNone,
    }
}

fn main() {
    let some: CustomOption<String> = example(true);
    let none: CustomOption<String> = example(false);

    assert!(some.is_some());
    assert!(!some.is_none());
    assert!(!none.is_some());
    assert!(none.is_none());
    assert_eq!("Hello world", some.unwrap());

    // This will display output in the console, which may look like the
    // application has failed, but as long as the test summary is successful,
    // it's no cause for concern
    assert!(catch_unwind(|| none.unwrap()).is_err());
}
