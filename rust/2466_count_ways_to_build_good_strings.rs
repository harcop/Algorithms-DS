/// LeetCode #2466 - Count Ways To Build Good Strings
fn count_good_strings(low: i32, high: i32, zero: i32, one: i32) -> i32 {
    const MOD: i32 = 1_000_000_007;
    let high = high as usize;
    let low = low as usize;
    let zero = zero as usize;
    let one = one as usize;
    let mut dp = vec![0; high + 1];
    dp[0] = 1;
    let mut answer = 0;

    for length in 1..=high {
        if length >= zero {
            dp[length] = (dp[length] + dp[length - zero]) % MOD;
        }
        if length >= one {
            dp[length] = (dp[length] + dp[length - one]) % MOD;
        }
        if length >= low {
            answer = (answer + dp[length]) % MOD;
        }
    }

    answer
}

fn main() {
    println!("{}", count_good_strings(3, 3, 1, 1));
}

#[cfg(test)]
mod tests {
    use super::count_good_strings;

    #[test]
    fn example_one() {
        assert_eq!(count_good_strings(3, 3, 1, 1), 8);
    }

    #[test]
    fn example_two() {
        assert_eq!(count_good_strings(2, 3, 1, 2), 5);
    }
}
