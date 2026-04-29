/// LeetCode #72 - Edit Distance
fn min_distance(word1: String, word2: String) -> i32 {
    let a = word1.as_bytes();
    let b = word2.as_bytes();
    let m = a.len();
    let n = b.len();
    let mut dp = vec![0i32; n + 1];
    for j in 0..=n {
        dp[j] = j as i32;
    }

    for i in 1..=m {
        let mut prev = dp[0];
        dp[0] = i as i32;
        for j in 1..=n {
            let temp = dp[j];
            if a[i - 1] == b[j - 1] {
                dp[j] = prev;
            } else {
                dp[j] = 1 + dp[j].min(dp[j - 1]).min(prev);
            }
            prev = temp;
        }
    }
    dp[n]
}

fn main() {
    println!("{}", min_distance("horse".to_string(), "ros".to_string()));
}

#[cfg(test)]
mod tests {
    use super::min_distance;

    #[test]
    fn example_one() {
        assert_eq!(min_distance("horse".to_string(), "ros".to_string()), 3);
    }

    #[test]
    fn example_two() {
        assert_eq!(min_distance("intention".to_string(), "execution".to_string()), 5);
    }
}
