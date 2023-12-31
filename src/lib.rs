#[derive(Debug, PartialEq)]
pub struct Fibonacci {
    fn_2: u8,
    fn_1: u8,
}

impl Default for Fibonacci {
    fn default() -> Self {
        Self { fn_2: 0, fn_1: 1 }
    }
}

impl Iterator for Fibonacci {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        let val = self.fn_2;
        self.fn_2 = self.fn_1;
        // Don't panic. Instead of overflowing, repeat the same number.
        self.fn_1 = match self.fn_1.checked_add(val) {
            Some(sum) => sum,
            None => self.fn_1,
        };
        Some(val)
    }
}

#[cfg(test)]
mod tests {
    use std::vec;

    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_init() {
        let f0 = Fibonacci { fn_2: 0, fn_1: 1 };

        assert_eq!(f0, Fibonacci::default());
    }

    #[test]
    fn test_sequence() {
        let fib = Fibonacci::default();

        let expected: Vec<u8> = vec![0, 1, 1, 2, 3];
        let result: Vec<u8> = fib.take(5).collect();

        // Take 5 fibonacci numbers and put them into a vector.
        assert_eq!(result, expected);
    }

    #[test]
    fn test_overflow_check() {
        let fib = Fibonacci::default();
        let sequence: Vec<u8> = fib.take(200).collect();
        assert_eq!(sequence[198], sequence[199]);
    }
}
