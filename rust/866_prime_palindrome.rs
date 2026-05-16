/// LeetCode #866 - Prime Palindrome
fn prime_palindrome(n: i32) -> i32 {
    fn is_palindrome(x: i32) -> bool {
        let s = x.to_string();
        s == s.chars().rev().collect::<String>()
    }
    fn is_prime(x: i32) -> bool {
        if x < 2 {
            return false;
        }
        let mut i = 2;
        while i * i <= x {
            if x % i == 0 {
                return false;
            }
            i += 1;
        }
        true
    }
    let mut x = n;
    while !(is_palindrome(x) && is_prime(x)) {
        x += 1;
    }
    x
}

fn main() {
    println!("{}", prime_palindrome(6));
}

#[cfg(test)]
mod tests {
    use super::prime_palindrome;

    #[test]
    fn example_one() {
        assert_eq!(prime_palindrome(6), 7);
    }

    #[test]
    fn example_two() {
        assert_eq!(prime_palindrome(8), 11);
    }
}
