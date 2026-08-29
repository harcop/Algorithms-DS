/// LeetCode #3472 - Longest Palindromic Subsequence After at Most K Operations
fn longest_palindromic_subsequence(s: String, k: i32) -> i32 {
    let s: Vec<i32> = s.bytes().map(|c| c as i32).collect();
    let n = s.len();
    let k = k as usize;
    let mut memo = vec![vec![vec![-1i32; k + 1]; n]; n];
    fn dfs(i: usize, j: usize, k: usize, s: &[i32], memo: &mut [Vec<Vec<i32>>]) -> i32 {
        if i > j {
            return 0;
        }
        if i == j {
            return 1;
        }
        if memo[i][j][k] != -1 {
            return memo[i][j][k];
        }
        let mut res = dfs(i + 1, j, k, s, memo).max(dfs(i, j - 1, k, s, memo));
        let d = (s[i] - s[j]).unsigned_abs() as usize;
        let t = d.min(26 - d);
        if t <= k {
            res = res.max(dfs(i + 1, j - 1, k - t, s, memo) + 2);
        }
        memo[i][j][k] = res;
        res
    }
    dfs(0, n - 1, k, &s, &mut memo)
}

fn main() {
    println!("{}", longest_palindromic_subsequence("abced".into(), 2));
}

#[cfg(test)]
mod tests {
    use super::longest_palindromic_subsequence;

    #[test]
    fn example1() {
        assert_eq!(longest_palindromic_subsequence("abced".into(), 2), 3);
    }

    #[test]
    fn example2() {
        assert_eq!(longest_palindromic_subsequence("aaazzz".into(), 4), 6);
    }
}
