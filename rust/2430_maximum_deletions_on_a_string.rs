/// LeetCode #2430 - Maximum Deletions on a String
fn delete_string(s: String) -> i32 {
    let bytes = s.as_bytes();
    let n = bytes.len();
    let mut lcp = vec![vec![0; n + 1]; n + 1];

    for i in (0..n).rev() {
        for j in (i + 1..n).rev() {
            if bytes[i] == bytes[j] {
                lcp[i][j] = lcp[i + 1][j + 1] + 1;
            }
        }
    }

    let mut dp = vec![1; n];
    for i in (0..n).rev() {
        for length in 1..=(n - i) / 2 {
            if lcp[i][i + length] >= length {
                dp[i] = dp[i].max(1 + dp[i + length]);
            }
        }
    }

    dp[0]
}

fn main() {
    println!("{}", delete_string("abcabcdabc".to_string()));
}

#[cfg(test)]
mod tests {
    use super::delete_string;

    #[test]
    fn example_one() {
        assert_eq!(delete_string("abcabcdabc".to_string()), 2);
    }

    #[test]
    fn example_two() {
        assert_eq!(delete_string("aaabaab".to_string()), 4);
    }

    #[test]
    fn all_same_letter() {
        assert_eq!(delete_string("aaaaa".to_string()), 5);
    }
}
