//! Provides the domain logic for calculating fizzbuzz
//!

pub trait FizzBuzzable {
    fn fizzbuzz(&self) -> String;
}

impl FizzBuzzable for i64 {
    fn fizzbuzz(&self) -> String {
        let mut result: String = "".to_string();
        if self % 3 == 0 {
            result += "fizz"
        }
        if self % 5 == 0 {
            result += "buzz";
        }
        if result == "" {
            result = self.to_string();
        }
        result.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn divisible_by_three_is_fizz() {
        let input = 6;
        let want = "fizz";
        let got = input.fizzbuzz();
        assert_eq!(want, got)
    }

    #[test]
    fn divisible_by_five_is_buzz() {
        let input = 10;
        let want = "buzz";
        let got = input.fizzbuzz();
        assert_eq!(want, got)
    }

    #[test]
    fn divisible_by_fifteen_is_fizzbuzz() {
        let input = 30;
        let want = "fizzbuzz";
        let got = input.fizzbuzz();
        assert_eq!(want, got)
    }

    #[test]
    fn divisible_by_neither_is_num_as_string() {
        let input = 7;
        let want = "7";
        let got = input.fizzbuzz();
        assert_eq!(want, got)
    }
}
