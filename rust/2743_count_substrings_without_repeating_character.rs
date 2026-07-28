/// LeetCode #2743 - Count Substrings Without Repeating Character
fn number_of_special_substrings(s: String) -> i64 {
    let bytes = s.as_bytes();
    let n = bytes.len();
    let mut cnt = [0i32; 26];
    let mut ans = 0i64;
    let mut j = 0;
    for i in 0..n {
        let k = (bytes[i] - b'a') as usize;
        cnt[k] += 1;
        while cnt[k] > 1 {
            let lk = (bytes[j] - b'a') as usize;
            cnt[lk] -= 1;
            j += 1;
        }
        ans += (i - j + 1) as i64;
    }
    ans
}

fn main() {
    println!("{}", number_of_special_substrings("abcd".into()));
}

#[cfg(test)]
mod tests {
    use super::number_of_special_substrings;

    #[test]
    fn example_one() {
        assert_eq!(number_of_special_substrings("abcd".into()), 10);
    }

    #[test]
    fn example_two() {
        assert_eq!(number_of_special_substrings("ooo".into()), 3);
    }

    #[test]
    fn example_three() {
        assert_eq!(number_of_special_substrings("abab".into()), 7);
    }
}
