/// LeetCode #1684 - Count The Number Of Consistent Strings
fn count_consistent_strings(allowed: String, words: Vec<String>) -> i32 {
    let mut mask = 0u32;
    for c in allowed.bytes() { mask |= 1 << (c - b'a'); }
    words.iter().filter(|w| w.bytes().all(|c| (mask >> (c - b'a')) & 1 == 1)).count() as i32
}
fn main() { println!("{}", count_consistent_strings("ab".into(), vec!["ad".into(),"bd".into(),"aaab".into(),"baa".into(),"badab".into()])); }
#[cfg(test)]
mod tests {
    use super::count_consistent_strings;
    #[test]
    fn example_one() { assert_eq!(count_consistent_strings("ab".into(), vec!["ad".into(),"bd".into(),"aaab".into(),"baa".into(),"badab".into()]), 2); }
}