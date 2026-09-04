/// LeetCode #828 - Count Unique Characters of All Substrings of a Given String
fn unique_letter_string(s: String) -> i32 {
    let bytes = s.as_bytes();
    let n = bytes.len();
    let mut last = [-1i32; 26];
    let mut prev = vec![-1i32; n];
    for i in 0..n {
        let c = (bytes[i] - b'A') as usize;
        prev[i] = last[c];
        last[c] = i as i32;
    }
    last = [n as i32; 26];
    let mut next = vec![n as i32; n];
    for i in (0..n).rev() {
        let c = (bytes[i] - b'A') as usize;
        next[i] = last[c];
        last[c] = i as i32;
    }
    let mut ans: i64 = 0;
    for i in 0..n {
        ans += (i as i32 - prev[i]) as i64 * (next[i] - i as i32) as i64;
    }
    ans as i32
}

fn main() {
    println!("{}", unique_letter_string("ABC".into()));
}

#[cfg(test)]
mod tests {
    use super::unique_letter_string;

    #[test]
    fn example_one() {
        assert_eq!(unique_letter_string("ABC".into()), 10);
    }

    #[test]
    fn example_two() {
        assert_eq!(unique_letter_string("ABA".into()), 8);
    }

    #[test]
    fn example_three() {
        assert_eq!(unique_letter_string("LEETCODE".into()), 92);
    }
}
