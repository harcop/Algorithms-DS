/// LeetCode #2266 - Count Number of Texts
const MOD: i64 = 1_000_000_007;

fn count_texts(pressed_keys: String) -> i32 {
    let chars: Vec<char> = pressed_keys.chars().collect();
    let n = chars.len();
    let mut dp = vec![0i64; n + 1];
    dp[n] = 1;

    for i in (0..n).rev() {
        dp[i] = dp[i + 1];
        if is_same(&chars, i, 2) {
            dp[i] += dp[i + 2];
        }
        if is_same(&chars, i, 3) {
            dp[i] += dp[i + 3];
        }
        if (chars[i] == '7' || chars[i] == '9') && is_same(&chars, i, 4) {
            dp[i] += dp[i + 4];
        }
        dp[i] %= MOD;
    }

    dp[0] as i32
}

fn is_same(chars: &[char], i: usize, k: usize) -> bool {
    if i + k > chars.len() {
        return false;
    }
    chars[i + 1..i + k].iter().all(|&c| c == chars[i])
}

fn main() {
    println!("{}", count_texts("22233".to_string()));
}

#[cfg(test)]
mod tests {
    use super::count_texts;

    #[test]
    fn example_one() {
        assert_eq!(count_texts("22233".to_string()), 8);
    }

    #[test]
    fn example_two() {
        assert_eq!(count_texts("2".to_string()), 1);
    }
}
