/// LeetCode #2911 - Minimum Changes to Make K Semi-palindromes
fn minimum_changes(s: String, k: i32) -> i32 {
    let n = s.len();
    let s = s.as_bytes();
    let k = k as usize;

    let mut factors = vec![vec![1]; n + 1];
    for d in 2..n {
        let mut i = d * 2;
        while i <= n {
            factors[i].push(d);
            i += d;
        }
    }

    let mut cost = vec![vec![0; n]; n];
    for i in 0..n {
        for j in i + 1..n {
            let length = j - i + 1;
            let mut min_cost = length as i32;
            for &d in &factors[length] {
                min_cost = min_cost.min(cost_d(s, i, j, d));
            }
            cost[i][j] = min_cost;
        }
    }

    let mut dp = vec![vec![n as i32; k + 1]; n + 1];
    dp[n][0] = 0;
    for i in (0..n).rev() {
        for j in 1..=k {
            for l in i + 1..n {
                dp[i][j] = dp[i][j].min(dp[l + 1][j - 1] + cost[i][l]);
            }
        }
    }
    dp[0][k]
}

fn cost_d(s: &[u8], i: usize, j: usize, d: usize) -> i32 {
    let mut cost = 0;
    for offset in 0..d {
        let mut l = i + offset;
        let mut r = j - d + 1 + offset;
        while l < r {
            if s[l] != s[r] {
                cost += 1;
            }
            l += d;
            r -= d;
        }
    }
    cost
}

fn main() {
    println!("{}", minimum_changes("abcac".into(), 2));
}

#[cfg(test)]
mod tests {
    use super::minimum_changes;

    #[test]
    fn example_one() {
        assert_eq!(minimum_changes("abcac".into(), 2), 1);
    }

    #[test]
    fn example_two() {
        assert_eq!(minimum_changes("abcdef".into(), 2), 2);
    }

    #[test]
    fn example_three() {
        assert_eq!(minimum_changes("aabbaa".into(), 3), 0);
    }
}
