use std::error;
use std::fmt;

#[allow(dead_code)]
type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

#[derive(Debug)]
struct EmptyVector;

impl fmt::Display for EmptyVector {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "invalid vector first element")
    }
}

impl error::Error for EmptyVector {
    fn description(&self) -> &str {
        "invalid vector first element"
    }
    fn cause(&self) -> Option<&dyn error::Error> {
        None
    }
}

#[derive(Clone, Debug)]
pub struct Fibonacci {
    current: u32,
    next: u32,
}

impl Iterator for Fibonacci {
    type Item = u32;

    fn next(&mut self) -> Option<u32> {
        let new_next = self.current + self.next;
        self.current = self.next;
        self.next = new_next;
        Some(new_next)
    }
}

#[allow(dead_code)]
pub fn fibonacci() -> Fibonacci {
    Fibonacci {
        current: 1,
        next: 1,
    }
}
