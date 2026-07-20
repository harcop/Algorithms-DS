/// LeetCode #2533 - Number of Good Binary Strings
fn good_binary_strings(min_length: i32, max_length: i32, one_group: i32, zero_group: i32) -> i32 {
    const MOD: i32 = 1_000_000_007;
    let max_length = max_length as usize;
    let min_length = min_length as usize;
    let one_group = one_group as usize;
    let zero_group = zero_group as usize;
    let mut dp = vec![0i32; max_length + 1];
    dp[0] = 1;
    for i in 1..=max_length {
        if i >= one_group {
            dp[i] = (dp[i] + dp[i - one_group]) % MOD;
        }
        if i >= zero_group {
            dp[i] = (dp[i] + dp[i - zero_group]) % MOD;
        }
    }
    let mut ans = 0i32;
    for i in min_length..=max_length {
        ans = (ans + dp[i]) % MOD;
    }
    ans
}

fn main() {
    println!("{}", good_binary_strings(2, 3, 1, 2));
}

#[cfg(test)]
mod tests {
    use super::good_binary_strings;

    #[test]
    fn example_one() {
        assert_eq!(good_binary_strings(2, 3, 1, 2), 5);
    }

    #[test]
    fn example_two() {
        assert_eq!(good_binary_strings(4, 4, 4, 3), 1);
    }
}
