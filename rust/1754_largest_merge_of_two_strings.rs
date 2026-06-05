/// LeetCode #1754 - Largest Merge Of Two Strings
fn pick_greater(i: usize, j: usize, w1: &[char], w2: &[char]) -> bool {
    let mut a = i;
    let mut b = j;
    while a < w1.len() && b < w2.len() {
        if w1[a] != w2[b] {
            return w1[a] > w2[b];
        }
        a += 1;
        b += 1;
    }
    a < w1.len()
}
fn largest_merge(word1: String, word2: String) -> String {
    let w1: Vec<char> = word1.chars().collect();
    let w2: Vec<char> = word2.chars().collect();
    let mut i = 0usize;
    let mut j = 0usize;
    let mut ans = String::new();
    while i < w1.len() && j < w2.len() {
        if pick_greater(i, j, &w1, &w2) {
            ans.push(w1[i]);
            i += 1;
        } else {
            ans.push(w2[j]);
            j += 1;
        }
    }
    ans.extend(w1[i..].iter());
    ans.extend(w2[j..].iter());
    ans
}
fn main() { println!("{}", largest_merge("abc".into(), "dcy".into())); }
#[cfg(test)]
mod tests {
    use super::largest_merge;
    #[test]
    fn example_one() {
        assert_eq!(largest_merge("abc".into(), "dcy".into()), "dcyabc");
    }
    #[test]
    fn example_two() {
        assert_eq!(largest_merge("ab".into(), "cd".into()), "cdab");
    }
}
