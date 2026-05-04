/// LeetCode #312 - Burst Balloons
fn max_coins(nums: Vec<i32>) -> i32 {
    let mut a = vec![1];
    a.extend(nums);
    a.push(1);
    let n = a.len();
    let mut dp = vec![vec![0; n]; n];
    for len in 2..n {
        for l in 0..n - len {
            let r = l + len;
            for k in l + 1..r {
                dp[l][r] = dp[l][r].max(dp[l][k] + dp[k][r] + a[l] * a[k] * a[r]);
            }
        }
    }
    dp[0][n - 1]
}

fn main() {
    println!("{}", max_coins(vec![3, 1, 5, 8]));
}

#[cfg(test)]
mod tests {
    use super::max_coins;

    #[test]
    fn example_one() {
        assert_eq!(max_coins(vec![3, 1, 5, 8]), 167);
    }
}
