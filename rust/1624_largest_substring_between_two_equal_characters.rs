/// LeetCode #1624 - Largest Substring Between Two Equal Characters
fn max_length_between_equal_characters(s: String) -> i32 {
    let mut last = [usize::MAX; 26];
    let mut ans = -1i32;
    for (i, c) in s.bytes().enumerate() {
        let k = (c - b'a') as usize;
        if last[k] != usize::MAX { ans = ans.max(i as i32 - last[k] as i32 - 1); }
        last[k] = i;
    }
    ans
}
fn main() { println!("{}", max_length_between_equal_characters("aa".into())); }
#[cfg(test)]
mod tests {
    use super::max_length_between_equal_characters;
    #[test]
    fn example_one() { assert_eq!(max_length_between_equal_characters("aa".into()), 0); }
}