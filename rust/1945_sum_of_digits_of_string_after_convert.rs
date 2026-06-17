/// LeetCode #1945 - Sum of Digits of String After Convert
fn get_lucky(s: String, k: i32) -> i32 {
    let mut digits = String::new();
    for b in s.bytes() {
        digits.push_str(&(b - b'a' + 1).to_string());
    }
    for _ in 0..k {
        let sum: u32 = digits.bytes().map(|b| (b - b'0') as u32).sum();
        digits = sum.to_string();
    }
    digits.parse().unwrap()
}

fn main() {
    println!("{}", get_lucky("iiii".into(), 1));
}

#[cfg(test)]
mod tests {
    use super::get_lucky;

    #[test]
    fn example_one() {
        assert_eq!(get_lucky("iiii".into(), 1), 36);
    }

    #[test]
    fn example_two() {
        assert_eq!(get_lucky("leetcode".into(), 2), 6);
    }

    #[test]
    fn example_three() {
        assert_eq!(get_lucky("zbax".into(), 2), 8);
    }
}
