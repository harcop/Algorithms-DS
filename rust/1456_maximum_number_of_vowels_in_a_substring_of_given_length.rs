/// LeetCode #1456 - Maximum Number Of Vowels In A Substring Of Given Length
fn is_vowel(c: u8) -> bool { matches!(c, b'a'|b'e'|b'i'|b'o'|b'u') }
fn max_vowels(s: String, k: i32) -> i32 {
    let b = s.as_bytes();
    let k = k as usize;
    let mut cur = b[..k].iter().filter(|&&c| is_vowel(c)).count() as i32;
    let mut best = cur;
    for i in k..b.len() {
        if is_vowel(b[i-k]) { cur -= 1; }
        if is_vowel(b[i]) { cur += 1; }
        best = best.max(cur);
    }
    best
}
fn main() { println!("{}", max_vowels("abciiidef".into(), 3)); }
#[cfg(test)]
mod tests {
    use super::max_vowels;
    #[test]
    fn example_one() { assert_eq!(max_vowels("abciiidef".into(), 3), 3); }
    #[test]
    fn example_two() { assert_eq!(max_vowels("aeiou".into(), 2), 2); }
}