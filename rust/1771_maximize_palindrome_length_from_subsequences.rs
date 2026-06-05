/// LeetCode #1771 - Maximize Palindrome Length From Subsequences
fn longest_palindrome(word1: String, word2: String) -> i32 {
    let s: Vec<char> = format!("{}{}", word1, word2).chars().collect();
    let n = s.len();
    let l1 = word1.len();
    let mut f = vec![vec![0i32; n]; n];
    for i in 0..n {
        f[i][i] = 1;
    }
    let mut ans = 0i32;
    for i in (0..n - 1).rev() {
        for j in i + 1..n {
            if s[i] == s[j] {
                f[i][j] = f[i + 1][j - 1] + 2;
                if i < l1 && j >= l1 {
                    ans = ans.max(f[i][j]);
                }
            } else {
                f[i][j] = f[i + 1][j].max(f[i][j - 1]);
            }
        }
    }
    ans
}
fn main() { println!("{}", longest_palindrome("cacb".into(), "cbba".into())); }
#[cfg(test)]
mod tests {
    use super::longest_palindrome;
    #[test]
    fn example_one() { assert_eq!(longest_palindrome("cacb".into(), "cbba".into()), 5); }
    #[test]
    fn example_two() { assert_eq!(longest_palindrome("ab".into(), "ab".into()), 3); }
}
