/// LeetCode #3743 - Maximize Cyclic Partition Score
fn max_score(nums: Vec<i32>, k: i32) -> i64 {
    let n = nums.len();
    let k = k as usize;
    let mut best = 0i64;
    let mut a = nums;
    for _ in 0..n {
        best = best.max(linear(&a, k));
        a.rotate_left(1);
    }
    best
}

fn linear(a: &[i32], k: usize) -> i64 {
    let n = a.len();
    let mut rng = vec![vec![0i64; n]; n];
    for i in 0..n {
        let mut mx = a[i];
        let mut mn = a[i];
        for j in i..n {
            mx = mx.max(a[j]);
            mn = mn.min(a[j]);
            rng[i][j] = (mx - mn) as i64;
        }
    }
    let mut dp = vec![vec![i64::MIN / 4; n + 1]; k + 1];
    dp[0][0] = 0;
    for p in 1..=k {
        for j in p..=n {
            for i in (p - 1)..j {
                if dp[p - 1][i] > i64::MIN / 8 {
                    dp[p][j] = dp[p][j].max(dp[p - 1][i] + rng[i][j - 1]);
                }
            }
        }
    }
    (1..=k).map(|p| dp[p][n]).max().unwrap()
}

fn main() {
    println!("{}", max_score(vec![1, 2, 3, 3], 2));
}

#[cfg(test)]
mod tests {
    use super::max_score;

    #[test]
    fn example1() {
        assert_eq!(max_score(vec![1, 2, 3, 3], 2), 3);
    }

    #[test]
    fn example2() {
        assert_eq!(max_score(vec![1, 2, 3, 3], 1), 2);
    }

    #[test]
    fn example3() {
        assert_eq!(max_score(vec![1, 2, 3, 3], 4), 3);
    }
}
