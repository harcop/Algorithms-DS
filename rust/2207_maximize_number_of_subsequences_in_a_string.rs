/// LeetCode #2207 - Maximize Number of Subsequences in a String
fn maximum_subsequence_count(text: String, pattern: String) -> i64 {
    let p = pattern.as_bytes();
    let mut ans = 0i64;
    let mut count0 = 0i64;
    let mut count1 = 0i64;

    for c in text.bytes() {
        if c == p[1] {
            ans += count0;
            count1 += 1;
        }
        if c == p[0] {
            count0 += 1;
        }
    }

    ans + count0.max(count1)
}

fn main() {
    println!("{}", maximum_subsequence_count("abdcdbc".into(), "ac".into()));
}

#[cfg(test)]
mod tests {
    use super::maximum_subsequence_count;

    #[test]
    fn example_one() {
        assert_eq!(maximum_subsequence_count("abdcdbc".into(), "ac".into()), 4);
    }

    #[test]
    fn example_two() {
        assert_eq!(maximum_subsequence_count("abcd".into(), "ab".into()), 2);
    }
}
