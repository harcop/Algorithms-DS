/// LeetCode #326 - Power of Three
fn is_power_of_three(n: i32) -> bool {
    n > 0 && 1162261467 % n == 0
}

fn main() {
    println!("{}", is_power_of_three(27));
}

#[cfg(test)]
mod tests {
    use super::is_power_of_three;

    #[test]
    fn example_one() {
        assert!(is_power_of_three(27));
    }

    #[test]
    fn example_two() {
        assert!(!is_power_of_three(0));
    }
}
