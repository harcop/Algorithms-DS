/// LeetCode #467 - Unique Substrings in Wraparound String
fn find_substring_in_wraparound_string(p: String) -> i32 {
    let mut len = [0i32; 26];
    let mut cur = 0i32;
    let b = p.as_bytes();
    for i in 0..b.len() {
        if i > 0 {
            let prev = b[i - 1];
            let now = b[i];
            let wrap = (prev == b'z' && now == b'a')
                || (now == prev + 1);
            if wrap {
                cur += 1;
            } else {
                cur = 1;
            }
        } else {
            cur = 1;
        }
        let j = (b[i] - b'a') as usize;
        len[j] = len[j].max(cur);
    }
    len.iter().sum()
}

fn main() {
    println!("{}", find_substring_in_wraparound_string("zab".into()));
}

#[cfg(test)]
mod tests {
    use super::find_substring_in_wraparound_string;

    #[test]
    fn example_one() {
        assert_eq!(find_substring_in_wraparound_string("a".into()), 1);
    }

    #[test]
    fn example_two() {
        assert_eq!(find_substring_in_wraparound_string("cac".into()), 2);
    }

    #[test]
    fn example_three() {
        assert_eq!(find_substring_in_wraparound_string("zab".into()), 6);
    }
}
