/// LeetCode #2311 - Longest Binary Subsequence Less Than or Equal to K
fn longest_subsequence(s: String, k: i32) -> i32 {
    let bytes = s.as_bytes();
    let mut ans = 0i32;
    let mut v = 0i32;
    for i in (0..bytes.len()).rev() {
        if bytes[i] == b'0' {
            ans += 1;
        } else if ans < 30 && (v | (1 << ans)) <= k {
            v |= 1 << ans;
            ans += 1;
        }
    }
    ans
}

fn main() {
    println!("{}", longest_subsequence("1001010".to_string(), 5));
}

#[cfg(test)]
mod tests {
    use super::longest_subsequence;

    #[test]
    fn example_one() {
        assert_eq!(longest_subsequence("1001010".to_string(), 5), 5);
    }

    #[test]
    fn example_two() {
        assert_eq!(longest_subsequence("00101001".to_string(), 1), 6);
    }
}
