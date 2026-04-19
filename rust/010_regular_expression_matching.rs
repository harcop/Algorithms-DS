/// LeetCode #10 - Regular Expression Matching
///
/// '.' matches any single character; '*' matches zero or more of the preceding element.
/// Full match of the entire input string is required.

fn is_match(s: String, p: String) -> bool {
    let s: Vec<char> = s.chars().collect();
    let p: Vec<char> = p.chars().collect();
    let mut memo = vec![vec![None; p.len() + 1]; s.len() + 1];

    fn dfs(
        i: usize,
        j: usize,
        s: &[char],
        p: &[char],
        memo: &mut Vec<Vec<Option<bool>>>,
    ) -> bool {
        if let Some(v) = memo[i][j] {
            return v;
        }

        let out = if j == p.len() {
            i == s.len()
        } else {
            let first = i < s.len() && (p[j] == s[i] || p[j] == '.');
            if j + 1 < p.len() && p[j + 1] == '*' {
                dfs(i, j + 2, s, p, memo) || (first && dfs(i + 1, j, s, p, memo))
            } else {
                first && dfs(i + 1, j + 1, s, p, memo)
            }
        };

        memo[i][j] = Some(out);
        out
    }

    dfs(0, 0, &s, &p, &mut memo)
}

fn main() {
    println!("{}", is_match("aa".to_string(), "a*".to_string()));
}

#[cfg(test)]
mod tests {
    use super::is_match;

    #[test]
    fn example_one() {
        assert!(!is_match("aa".to_string(), "a".to_string()));
    }

    #[test]
    fn example_two() {
        assert!(is_match("aa".to_string(), "a*".to_string()));
    }

    #[test]
    fn example_three() {
        assert!(is_match("ab".to_string(), ".*".to_string()));
    }
}
