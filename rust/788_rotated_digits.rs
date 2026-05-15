/// LeetCode #788 - Rotated Digits
fn rotated_digits(n: i32) -> i32 {
    fn good(mut x: i32) -> bool {
        let mut has = false;
        while x > 0 {
            let d = x % 10;
            match d {
                2 | 5 | 6 | 9 => has = true,
                0 | 1 | 8 => {}
                _ => return false,
            }
            x /= 10;
        }
        has
    }
    (1..=n).filter(|&x| good(x)).count() as i32
}

fn main() {
    println!("{}", rotated_digits(10));
}

#[cfg(test)]
mod tests {
    use super::rotated_digits;

    #[test]
    fn example_one() {
        assert_eq!(rotated_digits(10), 4);
    }

    #[test]
    fn example_two() {
        assert_eq!(rotated_digits(1), 0);
    }
}
