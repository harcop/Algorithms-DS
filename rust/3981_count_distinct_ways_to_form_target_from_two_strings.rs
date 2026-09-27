/// LeetCode #3981 - Count Distinct Ways to Form Target from Two Strings
fn count_ways(word1: String, word2: String, target: String) -> i32 {
    const MOD: i64 = 1_000_000_007;
    let a = word1.as_bytes();
    let b = word2.as_bytes();
    let t = target.as_bytes();
    let (n, m, p) = (a.len(), b.len(), t.len());
    let mut na = vec![[n; 26]; n + 1];
    let mut nb = vec![[m; 26]; m + 1];
    for i in (0..n).rev() {
        na[i] = na[i + 1];
        na[i][(a[i] - b'a') as usize] = i;
    }
    for i in (0..m).rev() {
        nb[i] = nb[i + 1];
        nb[i][(b[i] - b'a') as usize] = i;
    }
    let mut memo = vec![vec![vec![vec![-1i64; 4]; m + 1]; n + 1]; p + 1];
    fn dfs(
        i: usize,
        j: usize,
        k: usize,
        mask: usize,
        t: &[u8],
        na: &[[usize; 26]],
        nb: &[[usize; 26]],
        n: usize,
        m: usize,
        memo: &mut [Vec<Vec<Vec<i64>>>],
    ) -> i64 {
        if i == t.len() {
            return if mask == 3 { 1 } else { 0 };
        }
        if memo[i][j][k][mask] >= 0 {
            return memo[i][j][k][mask];
        }
        let c = (t[i] - b'a') as usize;
        let mut ans = 0i64;
        let p1 = na[j][c];
        if p1 < n {
            ans += dfs(i + 1, p1 + 1, k, mask | 1, t, na, nb, n, m, memo);
        }
        let p2 = nb[k][c];
        if p2 < m {
            ans += dfs(i + 1, j, p2 + 1, mask | 2, t, na, nb, n, m, memo);
        }
        ans %= MOD;
        memo[i][j][k][mask] = ans;
        ans
    }
    dfs(0, 0, 0, 0, t, &na, &nb, n, m, &mut memo) as i32
}

fn main() {
    println!("{}", count_ways("abc".into(), "bac".into(), "abc".into()));
}

#[cfg(test)]
mod tests {
    use super::count_ways;

    #[test]
    fn example1() {
        assert_eq!(count_ways("abc".into(), "bac".into(), "abc".into()), 5);
    }

    #[test]
    fn example2() {
        assert_eq!(count_ways("cd".into(), "cd".into(), "ccd".into()), 4);
    }

    #[test]
    fn example3() {
        assert_eq!(count_ways("xy".into(), "xy".into(), "xyxy".into()), 2);
    }

    #[test]
    fn example4() {
        assert_eq!(count_ways("ab".into(), "cde".into(), "ace".into()), 1);
    }
}
