/// LeetCode #2414 - Length of the Longest Alphabetical Continuous Substring
fn longest_continuous_substring(s: String) -> i32 {
    let bytes = s.as_bytes();
    let mut ans = 1;
    let mut cur = 1;

    for i in 1..bytes.len() {
        if bytes[i] == bytes[i - 1] + 1 {
            cur += 1;
        } else {
            cur = 1;
        }
        ans = ans.max(cur);
    }

    ans
}

fn main() {
    println!("{}", longest_continuous_substring("abacaba".to_string()));
}

#[cfg(test)]
mod tests {
    use super::longest_continuous_substring;

    #[test]
    fn example_one() {
        assert_eq!(longest_continuous_substring("abacaba".to_string()), 2);
    }

    #[test]
    fn example_two() {
        assert_eq!(longest_continuous_substring("abcde".to_string()), 5);
    }
}
