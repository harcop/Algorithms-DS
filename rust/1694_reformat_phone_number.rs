/// LeetCode #1694 - Reformat Phone Number
fn reformat_number(number: String) -> String {
    let digits: String = number.chars().filter(|c| c.is_ascii_digit()).collect();
    let n = digits.len();
    let mut ans = String::new();
    let mut i = 0usize;
    let mut block = 3usize;
    while i < n {
        if !ans.is_empty() { ans.push('-'); }
        let take = if block == 3 && (n - i) % 4 == 1 && n - i >= 4 { 3 } else if block == 3 && (n - i) % 4 == 0 { 3 } else if n - i <= 4 { n - i } else { 3 };
        let take = if n - i > 4 && (n - i) % 4 == 1 { 3 } else if n - i <= 3 { n - i } else { 3 };
        let rem = n - i;
        let take = if rem > 4 && rem % 4 == 1 { 3 } else if rem <= 3 { rem } else { 3 };
        ans.push_str(&digits[i..i + take]);
        i += take;
    }
    ans
}
fn main() { println!("{}", reformat_number("1-23-45 6".into())); }
#[cfg(test)]
mod tests {
    use super::reformat_number;
    #[test]
    fn example_one() { assert_eq!(reformat_number("1-23-45 6".into()), "123-456"); }
}