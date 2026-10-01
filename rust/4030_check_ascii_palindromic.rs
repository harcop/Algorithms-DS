/// LeetCode #4030 - Check ASCII Palindromic
fn is_palindromic(s: String) -> bool {
    let bits: Vec<u8> = s
        .bytes()
        .flat_map(|b| (0..8).rev().map(move |i| (b >> i) & 1))
        .collect();
    bits.iter().eq(bits.iter().rev())
}

fn main() {
    println!("{}", is_palindromic("ff".to_string()));
}

#[cfg(test)]
mod tests {
    use super::is_palindromic;

    #[test]
    fn example1() {
        assert!(is_palindromic("ff".to_string()));
    }

    #[test]
    fn example2() {
        assert!(!is_palindromic("leet".to_string()));
    }
}
