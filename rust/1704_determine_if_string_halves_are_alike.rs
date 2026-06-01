/// LeetCode #1704 - Determine If String Halves Are Alike
fn halves_are_alike(s: String) -> bool {
    let vowels: std::collections::HashSet<u8> = "aeiouAEIOU".bytes().collect();
    let b = s.as_bytes();
    let n = b.len() / 2;
    let a: i32 = b[..n].iter().filter(|&&c| vowels.contains(&c)).count() as i32;
    let c: i32 = b[n..].iter().filter(|&&c| vowels.contains(&c)).count() as i32;
    a == c
}
fn main() { println!("{}", halves_are_alike("book".into())); }
#[cfg(test)]
mod tests {
    use super::halves_are_alike;
    #[test]
    fn example_one() { assert!(halves_are_alike("book".into())); }
}