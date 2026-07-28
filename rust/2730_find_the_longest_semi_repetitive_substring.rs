/// LeetCode #2730 - Find the Longest Semi-Repetitive Substring
fn longest_semi_repetitive_substring(s: String) -> i32 {
    let bytes = s.as_bytes();
    let n = bytes.len();
    let mut ans = 1;
    let mut j = 0;
    let mut cnt = 0;
    for i in 1..n {
        if bytes[i] == bytes[i - 1] {
            cnt += 1;
        }
        while cnt > 1 {
            if bytes[j] == bytes[j + 1] {
                cnt -= 1;
            }
            j += 1;
        }
        ans = ans.max(i - j + 1);
    }
    ans as i32
}

fn main() {
    println!("{}", longest_semi_repetitive_substring("52233".into()));
}

#[cfg(test)]
mod tests {
    use super::longest_semi_repetitive_substring;

    #[test]
    fn example_one() {
        assert_eq!(longest_semi_repetitive_substring("52233".into()), 4);
    }

    #[test]
    fn example_two() {
        assert_eq!(longest_semi_repetitive_substring("5494".into()), 4);
    }

    #[test]
    fn example_three() {
        assert_eq!(longest_semi_repetitive_substring("1111111".into()), 2);
    }
}
