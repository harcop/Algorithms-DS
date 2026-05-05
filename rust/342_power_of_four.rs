/// LeetCode #342 - Power of Four
fn is_power_of_four(n: i32) -> bool {
    n > 0 && (n & (n - 1)) == 0 && (n & 0x5555_5555) != 0
}

fn main() {
    println!("{}", is_power_of_four(16));
}

#[cfg(test)]
mod tests {
    use super::is_power_of_four;

    #[test]
    fn example_one() {
        assert!(is_power_of_four(16));
    }

    #[test]
    fn example_two() {
        assert!(!is_power_of_four(8));
    }
}
