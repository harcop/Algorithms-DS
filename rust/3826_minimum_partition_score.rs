/// LeetCode #3826 - Minimum Partition Score
fn minimum_score(nums: Vec<i32>, k: i32) -> i64 {
    let n = nums.len();
    let k = k as usize;
    let mut p = vec![0i64; n + 1];
    for i in 0..n {
        p[i + 1] = p[i] + nums[i] as i64;
    }
    let inf = i64::MAX / 4;
    let mut dp = vec![inf; n + 1];
    dp[0] = 0;
    for _ in 0..k {
        let prev = dp;
        let mut ndp = vec![inf; n + 1];
        fn rec(
            l: usize,
            r: usize,
            optl: usize,
            optr: usize,
            prev: &[i64],
            ndp: &mut [i64],
            p: &[i64],
        ) {
            if l > r {
                return;
            }
            let mid = (l + r) / 2;
            let mut best = i64::MAX / 4;
            let mut best_j = optl;
            let hi = optr.min(mid.saturating_sub(1));
            for j in optl..=hi {
                if prev[j] >= i64::MAX / 8 {
                    continue;
                }
                let s = p[mid] - p[j];
                let cost = prev[j] + s * (s + 1) / 2;
                if cost < best {
                    best = cost;
                    best_j = j;
                }
            }
            ndp[mid] = best;
            if mid > 0 {
                rec(l, mid.saturating_sub(1), optl, best_j, prev, ndp, p);
            }
            rec(mid + 1, r, best_j, optr, prev, ndp, p);
        }
        rec(1, n, 0, n, &prev, &mut ndp, &p);
        dp = ndp;
    }
    dp[n]
}

fn main() {
    println!("{}", minimum_score(vec![5, 1, 2, 1], 2));
}

#[cfg(test)]
mod tests {
    use super::minimum_score;

    #[test]
    fn example1() {
        assert_eq!(minimum_score(vec![5, 1, 2, 1], 2), 25);
    }

    #[test]
    fn example2() {
        assert_eq!(minimum_score(vec![1, 2, 3, 4], 1), 55);
    }

    #[test]
    fn example3() {
        assert_eq!(minimum_score(vec![1, 1, 1], 3), 3);
    }
}
