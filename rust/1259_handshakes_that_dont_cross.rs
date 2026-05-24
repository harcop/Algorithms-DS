/// LeetCode #1259 - Handshakes That Don't Cross
const MOD: i64 = 1_000_000_007;

fn number_of_ways(num_people: i32) -> i32 {
    let n = num_people as usize / 2;
    let mut dp = vec![1i64; n + 1];
    for i in 1..=n {
        let mut sum = 0i64;
        for j in 0..i {
            sum = (sum + dp[j] * dp[i - 1 - j]) % MOD;
        }
        dp[i] = sum;
    }
    dp[n] as i32
}

fn main() {
    println!("{}", number_of_ways(4));
}

#[cfg(test)]
mod tests {
    use super::number_of_ways;

    #[test]
    fn example_one() {
        assert_eq!(number_of_ways(2), 1);
    }

    #[test]
    fn example_two() {
        assert_eq!(number_of_ways(4), 2);
    }
}
