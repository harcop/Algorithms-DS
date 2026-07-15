/// LeetCode #2396 - Strictly Palindromic Number
fn is_strictly_palindromic(_n: i32) -> bool {
    false
}

fn main() {
    println!("{}", is_strictly_palindromic(9));
}

#[cfg(test)]
mod tests {
    use super::is_strictly_palindromic;

    #[test]
    fn example_one() {
        assert!(!is_strictly_palindromic(9));
    }

    #[test]
    fn example_two() {
        assert!(!is_strictly_palindromic(4));
    }
}
