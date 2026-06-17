/// LeetCode #1930 - Unique Length-3 Palindromic Subsequences
fn count_palindromic_subsequence(s: String) -> i32 {
    let bytes = s.as_bytes();
    let n = bytes.len();
    let mut first = [n; 26];
    let mut last = [0usize; 26];
    for (i, &b) in bytes.iter().enumerate() {
        let idx = (b - b'a') as usize;
        first[idx] = first[idx].min(i);
        last[idx] = i;
    }
    let mut ans = 0;
    for i in 0..26 {
        if first[i] < last[i] {
            let mut seen = [false; 26];
            for &b in &bytes[first[i] + 1..last[i]] {
                let j = (b - b'a') as usize;
                if !seen[j] {
                    seen[j] = true;
                    ans += 1;
                }
            }
        }
    }
    ans
}

fn main() {
    println!("{}", count_palindromic_subsequence("aabca".into()));
}

#[cfg(test)]
mod tests {
    use super::count_palindromic_subsequence;

    #[test]
    fn example_one() {
        assert_eq!(count_palindromic_subsequence("aabca".into()), 3);
    }

    #[test]
    fn example_two() {
        assert_eq!(count_palindromic_subsequence("adc".into()), 0);
    }

    #[test]
    fn example_three() {
        assert_eq!(count_palindromic_subsequence("bbcbaba".into()), 4);
    }
}
