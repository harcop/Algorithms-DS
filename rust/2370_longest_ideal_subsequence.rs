/// LeetCode #2370 - Longest Ideal Subsequence
fn longest_ideal_string(s: String, k: i32) -> i32 {
    let mut dp = [0i32; 26];

    for c in s.bytes() {
        let i = (c - b'a') as usize;
        let first = i.saturating_sub(k as usize);
        let last = (i + k as usize).min(25);
        let max_reachable = dp[first..=last].iter().copied().max().unwrap_or(0);
        dp[i] = 1 + max_reachable;
    }

    *dp.iter().max().unwrap()
}

fn main() {
    println!("{}", longest_ideal_string("acfgbd".to_string(), 2));
}

#[cfg(test)]
mod tests {
    use super::longest_ideal_string;

    #[test]
    fn example_one() {
        assert_eq!(longest_ideal_string("acfgbd".to_string(), 2), 4);
    }

    #[test]
    fn example_two() {
        assert_eq!(longest_ideal_string("abcd".to_string(), 3), 4);
    }
}
