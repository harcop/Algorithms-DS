/// LeetCode #2119 - A Number After a Double Reversal
fn is_same_after_reversals(num: i32) -> bool {
    num == 0 || num % 10 != 0
}

fn main() {
    println!("{}", is_same_after_reversals(526));
}

#[cfg(test)]
mod tests {
    use super::is_same_after_reversals;

    #[test]
    fn example_one() {
        assert!(is_same_after_reversals(526));
    }

    #[test]
    fn example_two() {
        assert!(!is_same_after_reversals(1800));
    }

    #[test]
    fn example_three() {
        assert!(is_same_after_reversals(0));
    }
}
