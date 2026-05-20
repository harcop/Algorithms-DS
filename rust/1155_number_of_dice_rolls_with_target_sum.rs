/// LeetCode #1155 - Number of Dice Rolls With Target Sum
fn num_rolls_to_target(n: i32, k: i32, target: i32) -> i32 {
    const MOD: i32 = 1_000_000_007;
    let mut dp = vec![0i32; (target + 1) as usize];
    dp[0] = 1;
    for _ in 0..n {
        let mut ndp = vec![0i32; (target + 1) as usize];
        for s in 0..=target {
            if dp[s as usize] == 0 {
                continue;
            }
            for face in 1..=k {
                let ns = s + face;
                if ns <= target {
                    ndp[ns as usize] = (ndp[ns as usize] + dp[s as usize]) % MOD;
                }
            }
        }
        dp = ndp;
    }
    dp[target as usize]
}

fn main() {
    println!("{}", num_rolls_to_target(1, 6, 3));
}

#[cfg(test)]
mod tests {
    use super::num_rolls_to_target;

    #[test]
    fn example_one() {
        assert_eq!(num_rolls_to_target(1, 6, 3), 1);
    }

    #[test]
    fn example_two() {
        assert_eq!(num_rolls_to_target(2, 6, 7), 6);
    }
}
