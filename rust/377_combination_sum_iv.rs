/// LeetCode #377 - Combination Sum IV (order matters; ascending DP counter)
fn combination_sum_iv(mut nums: Vec<i32>, target: i32) -> i32 {
    let t = target as usize;
    let mut dp = vec![0i64; t + 1];
    dp[0] = 1;
    nums.sort_unstable();
    for s in 1..=t {
        for &x in &nums {
            if x as usize <= s {
                dp[s] += dp[s - x as usize];
            }
        }
    }
    dp[t] as i32
}

fn main() {
    println!("{}", combination_sum_iv(vec![1, 2, 3], 4));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lc() {
        assert_eq!(combination_sum_iv(vec![1, 2, 3], 4), 7);
    }
}
