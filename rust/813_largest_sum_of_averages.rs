/// LeetCode #813 - Largest Sum of Averages
fn largest_sum_of_averages(nums: Vec<i32>, k: i32) -> f64 {
    let n = nums.len();
    let mut prefix = vec![0.0f64; n + 1];
    for i in 0..n {
        prefix[i + 1] = prefix[i] + nums[i] as f64;
    }
    let mut dp = vec![vec![0.0f64; k as usize + 1]; n + 1];
    for i in 1..=n {
        dp[i][1] = (prefix[i] - prefix[0]) / i as f64;
    }
    for parts in 2..=k as usize {
        for i in parts..=n {
            for j in parts - 1..i {
                let avg = (prefix[i] - prefix[j]) / (i - j) as f64;
                dp[i][parts] = dp[i][parts].max(dp[j][parts - 1] + avg);
            }
        }
    }
    dp[n][k as usize]
}

fn main() {
    println!("{}", largest_sum_of_averages(vec![9, 1, 2, 3, 9], 3));
}

#[cfg(test)]
mod tests {
    use super::largest_sum_of_averages;

    #[test]
    fn example_one() {
        let v = largest_sum_of_averages(vec![9, 1, 2, 3, 9], 3);
        assert!((v - 20.0).abs() < 1e-5);
    }
}
