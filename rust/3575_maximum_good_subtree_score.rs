/// LeetCode #3575 - Maximum Good Subtree Score
const MOD: i64 = 1_000_000_007;
const NEG: i64 = i64::MIN / 4;
const M: usize = 1 << 10;

fn digit_mask(mut x: i32) -> Option<usize> {
    let mut mask = 0usize;
    while x > 0 {
        let bit = 1 << (x % 10);
        if mask & bit != 0 {
            return None;
        }
        mask |= bit;
        x /= 10;
    }
    Some(mask)
}

fn merge(a: &[i64], b: &[i64]) -> Vec<i64> {
    let mut c = vec![NEG; M];
    for i in 0..M {
        if a[i] == NEG {
            continue;
        }
        for j in 0..M {
            if b[j] == NEG || (i & j) != 0 {
                continue;
            }
            c[i | j] = c[i | j].max(a[i] + b[j]);
        }
    }
    c
}

fn good_subtree_sum(vals: Vec<i32>, par: Vec<i32>) -> i32 {
    let n = vals.len();
    let mut g = vec![Vec::new(); n];
    for i in 1..n {
        g[par[i] as usize].push(i);
    }
    let mut ans = 0i64;
    fn dfs(
        u: usize,
        g: &[Vec<usize>],
        vals: &[i32],
        ans: &mut i64,
    ) -> Vec<i64> {
        let mut dp = vec![NEG; M];
        dp[0] = 0;
        for &v in &g[u] {
            let child = dfs(v, g, vals, ans);
            dp = merge(&dp, &child);
        }
        if let Some(um) = digit_mask(vals[u]) {
            let old = dp.clone();
            for m in 0..M {
                if old[m] != NEG && (m & um) == 0 {
                    dp[m | um] = dp[m | um].max(old[m] + vals[u] as i64);
                }
            }
        }
        let mut best = 0;
        for &x in &dp {
            if x > best {
                best = x;
            }
        }
        *ans = (*ans + best) % MOD;
        dp
    }
    dfs(0, &g, &vals, &mut ans);
    ans as i32
}

fn main() {
    println!("{}", good_subtree_sum(vec![2, 3], vec![-1, 0]));
}

#[cfg(test)]
mod tests {
    use super::good_subtree_sum;

    #[test]
    fn example1() {
        assert_eq!(good_subtree_sum(vec![2, 3], vec![-1, 0]), 8);
    }

    #[test]
    fn example2() {
        assert_eq!(good_subtree_sum(vec![1, 5, 2], vec![-1, 0, 0]), 15);
    }

    #[test]
    fn example3() {
        assert_eq!(good_subtree_sum(vec![34, 1, 2], vec![-1, 0, 1]), 42);
    }

    #[test]
    fn example4() {
        assert_eq!(good_subtree_sum(vec![3, 22, 5], vec![-1, 0, 1]), 18);
    }
}
