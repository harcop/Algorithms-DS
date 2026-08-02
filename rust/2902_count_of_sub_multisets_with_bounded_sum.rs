/// LeetCode #2902 - Count of Sub-Multisets With Bounded Sum
fn count_sub_multisets(nums: Vec<i32>, l: i32, r: i32) -> i32 {
    use std::collections::HashMap;

    const MOD: i64 = 1_000_000_007;
    let r = r as usize;
    let l = l as usize;
    let mut dp = vec![0i64; r + 1];
    dp[0] = 1;

    let mut count = HashMap::new();
    for num in nums {
        *count.entry(num).or_insert(0) += 1;
    }
    let zeros = count.remove(&0).unwrap_or(0);

    for (num, freq) in count {
        let num = num as usize;
        let mut stride = dp.clone();
        for i in num..=r {
            stride[i] = (stride[i] + stride[i - num]) % MOD;
        }
        for i in (1..=r).rev() {
            if i >= num * (freq + 1) {
                dp[i] = (stride[i] - stride[i - num * (freq + 1)] + MOD) % MOD;
            } else {
                dp[i] = stride[i] % MOD;
            }
        }
    }

    let mut ans = 0i64;
    for value in dp.iter().take(r + 1).skip(l) {
        ans = (ans + value) % MOD;
    }
    ((zeros as i64 + 1) * ans % MOD) as i32
}

fn main() {
    println!("{}", count_sub_multisets(vec![1, 2, 2, 3], 6, 6));
}

#[cfg(test)]
mod tests {
    use super::count_sub_multisets;

    #[test]
    fn example_one() {
        assert_eq!(count_sub_multisets(vec![1, 2, 2, 3], 6, 6), 1);
    }

    #[test]
    fn example_two() {
        assert_eq!(count_sub_multisets(vec![2, 1, 4, 2, 7], 1, 5), 7);
    }

    #[test]
    fn example_three() {
        assert_eq!(count_sub_multisets(vec![1, 2, 1, 3, 5, 2], 3, 5), 9);
    }
}
