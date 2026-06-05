/// LeetCode #1745 - Palindrome Partitioning IV
fn check_partitioning(s: String) -> bool {
    let s: Vec<u8> = s.into_bytes();
    let n = s.len();
    let mut pal = vec![vec![true; n]; n];
    for i in (0..n).rev() {
        for j in i + 1..n {
            pal[i][j] = s[i] == s[j] && pal[i + 1][j - 1];
        }
    }
    for i in 0..n - 2 {
        for j in i + 1..n - 1 {
            if pal[0][i] && pal[i + 1][j] && pal[j + 1][n - 1] {
                return true;
            }
        }
    }
    false
}
fn main() { println!("{}", check_partitioning("abcbdd".into())); }
#[cfg(test)]
mod tests {
    use super::check_partitioning;
    #[test]
    fn example_one() { assert!(check_partitioning("abcbdd".into())); }
    #[test]
    fn example_two() { assert!(!check_partitioning("bcbddxy".into())); }
}
