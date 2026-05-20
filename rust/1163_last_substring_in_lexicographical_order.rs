/// LeetCode #1163 - Last Substring in Lexicographical Order
fn last_substring(s: String) -> String {
    let b = s.as_bytes();
    let n = b.len();
    let mut i = 0usize;
    let mut j = 1usize;
    let mut k = 0usize;
    while j + k < n {
        if b[i + k] == b[j + k] {
            k += 1;
            continue;
        }
        if b[i + k] < b[j + k] {
            i = j.max(i + k + 1);
            j = i + 1;
        } else {
            j = j + k + 1;
        }
        k = 0;
    }
    s[i..].to_string()
}

fn main() {
    println!("{}", last_substring("abab".to_string()));
}

#[cfg(test)]
mod tests {
    use super::last_substring;

    #[test]
    fn example_one() {
        assert_eq!(last_substring("abab".to_string()), "bab");
    }

    #[test]
    fn example_two() {
        assert_eq!(last_substring("leetcode".to_string()), "tcode");
    }
}
