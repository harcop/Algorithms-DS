/// LeetCode #2478 - Number of Beautiful Partitions
fn beautiful_partitions(s: String, k: i32, min_length: i32) -> i32 {
    fn is_prime(c: u8) -> bool {
        matches!(c, b'2' | b'3' | b'5' | b'7')
    }

    let chars = s.into_bytes();
    let n = chars.len();
    let k = k as usize;
    let min_length = min_length as usize;

    if n == 0 || !is_prime(chars[0]) || is_prime(chars[n - 1]) {
        return 0;
    }

    const MOD: i32 = 1_000_000_007;
    let mut memo = vec![vec![-1; k]; n + 1];

    fn dfs(i: usize, bars: usize, chars: &[u8], min_length: usize, memo: &mut [Vec<i32>]) -> i32 {
        if bars == 0 {
            return if i <= chars.len() { 1 } else { 0 };
        }
        if i >= chars.len() {
            return 0;
        }
        if memo[i][bars] != -1 {
            return memo[i][bars];
        }

        let mut answer = dfs(i + 1, bars, chars, min_length, memo);
        if is_prime(chars[i]) && !is_prime(chars[i - 1]) {
            answer = (answer + dfs(i + min_length, bars - 1, chars, min_length, memo)) % MOD;
        }

        memo[i][bars] = answer % MOD;
        memo[i][bars]
    }

    dfs(min_length, k - 1, &chars, min_length, &mut memo)
}

fn main() {
    println!("{}", beautiful_partitions("23542185131".to_string(), 3, 2));
}

#[cfg(test)]
mod tests {
    use super::beautiful_partitions;

    #[test]
    fn example_one() {
        assert_eq!(beautiful_partitions("23542185131".to_string(), 3, 2), 3);
    }

    #[test]
    fn example_two() {
        assert_eq!(beautiful_partitions("23542185131".to_string(), 3, 3), 1);
    }

    #[test]
    fn invalid_start() {
        assert_eq!(beautiful_partitions("3312958".to_string(), 3, 1), 1);
    }
}
