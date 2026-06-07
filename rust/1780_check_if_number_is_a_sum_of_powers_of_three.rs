/// LeetCode #1780 - Check If Number Is a Sum of Powers of Three
fn check_powers_of_three(n: i32) -> bool {
    let mut n = n;
    while n > 0 {
        if n % 3 == 2 {
            return false;
        }
        n /= 3;
    }
    true
}
fn main() { println!("{}", check_powers_of_three(12)); }
#[cfg(test)]
mod tests {
    use super::check_powers_of_three;
    #[test]
    fn example_one() {
        assert!(check_powers_of_three(12));
    }
    #[test]
    fn example_two() {
        assert!(check_powers_of_three(91));
    }
    #[test]
    fn example_three() {
        assert!(!check_powers_of_three(21));
    }
}
