/// LeetCode #2704 - To Be Or Not To Be (JS problem; Rust Result analogue)
#[derive(Debug, PartialEq)]
enum ExpectError {
    NotEqual,
    Equal,
}

struct Expect<T> {
    val: T,
}

fn expect<T>(val: T) -> Expect<T> {
    Expect { val }
}

impl<T: PartialEq> Expect<T> {
    fn to_be(self, other: T) -> Result<bool, ExpectError> {
        if self.val != other {
            Err(ExpectError::NotEqual)
        } else {
            Ok(true)
        }
    }

    fn not_to_be(self, other: T) -> Result<bool, ExpectError> {
        if self.val == other {
            Err(ExpectError::Equal)
        } else {
            Ok(true)
        }
    }
}

fn main() {
    println!("{:?}", expect(5).to_be(5));
}

#[cfg(test)]
mod tests {
    use super::{expect, ExpectError};

    #[test]
    fn example_one() {
        assert_eq!(expect(5).to_be(5), Ok(true));
    }

    #[test]
    fn example_two() {
        assert_eq!(expect(5).to_be(0), Err(ExpectError::NotEqual));
    }

    #[test]
    fn example_three() {
        assert_eq!(expect(5).not_to_be(0), Ok(true));
    }

    #[test]
    fn not_to_be_equal() {
        assert_eq!(expect(5).not_to_be(5), Err(ExpectError::Equal));
    }
}
