/// LeetCode #131 - Palindrome Partitioning
fn partition(s: String) -> Vec<Vec<String>> {
    let b = s.as_bytes();
    let n = b.len();
    let mut dp = vec![vec![false; n]; n];
    for i in (0..n).rev() {
        for j in i..n {
            dp[i][j] = b[i] == b[j] && (j - i < 2 || dp[i + 1][j - 1]);
        }
    }
    let mut out = Vec::new();
    let mut path = Vec::new();
    fn backtrack(
        s: &str,
        start: usize,
        dp: &[Vec<bool>],
        path: &mut Vec<String>,
        out: &mut Vec<Vec<String>>,
    ) {
        if start == s.len() {
            out.push(path.clone());
            return;
        }
        for end in start..s.len() {
            if dp[start][end] {
                path.push(s[start..=end].to_string());
                backtrack(s, end + 1, dp, path, out);
                path.pop();
            }
        }
    }
    backtrack(&s, 0, &dp, &mut path, &mut out);
    out
}

fn main() {
    println!("{:?}", partition("aab".to_string()));
}

#[cfg(test)]
mod tests {
    use super::partition;

    fn normalize(mut v: Vec<Vec<String>>) -> Vec<Vec<String>> {
        v.sort();
        v
    }

    #[test]
    fn example_one() {
        let got = normalize(partition("aab".to_string()));
        let expected = normalize(vec![
            vec!["a".to_string(), "a".to_string(), "b".to_string()],
            vec!["aa".to_string(), "b".to_string()],
        ]);
        assert_eq!(got, expected);
    }

    #[test]
    fn example_two() {
        assert_eq!(partition("a".to_string()), vec![vec!["a".to_string()]]);
    }
}
