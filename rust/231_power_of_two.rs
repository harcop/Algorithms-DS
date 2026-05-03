/// LeetCode #231 - Power of Two
fn is_power_of_two(n: i32) -> bool {
    n > 0 && (n & (n - 1)) == 0
}

fn main() {
    println!("{}", is_power_of_two(16));
}

#[cfg(test)]
mod tests {
    use super::is_power_of_two;

    #[test]
    fn example_one() {
        assert!(is_power_of_two(1));
    }

    #[test]
    fn example_two() {
        assert!(is_power_of_two(16));
    }

    #[test]
    fn example_three() {
        assert!(!is_power_of_two(3));
    }
}
