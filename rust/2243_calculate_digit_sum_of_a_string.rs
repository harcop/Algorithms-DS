/// LeetCode #2243 - Calculate Digit Sum of a String
fn digit_sum(s: String, k: i32) -> String {
    let k = k as usize;
    let mut s = s;

    while s.len() > k {
        let chars: Vec<char> = s.chars().collect();
        let mut next = String::new();
        for i in (0..chars.len()).step_by(k) {
            let sum: u32 = chars[i..chars.len().min(i + k)]
                .iter()
                .map(|c| c.to_digit(10).unwrap())
                .sum();
            next.push_str(&sum.to_string());
        }
        s = next;
    }

    s
}

fn main() {
    println!("{}", digit_sum("11111222223".to_string(), 3));
}

#[cfg(test)]
mod tests {
    use super::digit_sum;

    #[test]
    fn example_one() {
        assert_eq!(digit_sum("11111222223".to_string(), 3), "135");
    }

    #[test]
    fn example_two() {
        assert_eq!(digit_sum("00000000".to_string(), 3), "000");
    }
}
