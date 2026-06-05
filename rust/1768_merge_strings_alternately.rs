/// LeetCode #1768 - Merge Strings Alternately
fn merge_alternately(word1: String, word2: String) -> String {
    let a: Vec<char> = word1.chars().collect();
    let b: Vec<char> = word2.chars().collect();
    let mut ans = String::new();
    let mut i = 0usize;
    let mut j = 0usize;
    while i < a.len() || j < b.len() {
        if i < a.len() {
            ans.push(a[i]);
            i += 1;
        }
        if j < b.len() {
            ans.push(b[j]);
            j += 1;
        }
    }
    ans
}
fn main() { println!("{}", merge_alternately("abc".into(), "pqr".into())); }
#[cfg(test)]
mod tests {
    use super::merge_alternately;
    #[test]
    fn example_one() { assert_eq!(merge_alternately("abc".into(), "pqr".into()), "apbqcr"); }
    #[test]
    fn example_two() { assert_eq!(merge_alternately("ab".into(), "pqrs".into()), "apbqrs"); }
}
