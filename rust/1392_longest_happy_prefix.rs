/// LeetCode #1392 - Longest Happy Prefix
fn longest_prefix(s: String) -> String {
    let b = s.as_bytes();
    let n = b.len();
    let mut lps = vec![0usize; n];
    let mut len = 0usize;
    let mut i = 1usize;
    while i < n {
        if b[i] == b[len] {
            len += 1;
            lps[i] = len;
            i += 1;
        } else if len > 0 {
            len = lps[len - 1];
        } else {
            lps[i] = 0;
            i += 1;
        }
    }
    s[..lps[n - 1]].to_string()
}

fn main() {
    println!("{}", longest_prefix("level".into()));
}

#[cfg(test)]
mod tests {
    use super::longest_prefix;

    #[test]
    fn example_one() {
        assert_eq!(longest_prefix("level".into()), "l");
    }

    #[test]
    fn example_two() {
        assert_eq!(longest_prefix("ababab".into()), "abab");
    }
}

