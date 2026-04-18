/// LeetCode #5 - Longest Palindromic Substring
///
/// Given a string s, return the longest palindromic substring in s.
/// Constraints mention English letters and digits (ASCII).

fn longest_palindrome(s: String) -> String {
    let b = s.as_bytes();
    let n = b.len();
    if n == 0 {
        return String::new();
    }

    let mut start = 0usize;
    let mut max_len = 1usize;

    let expand = |left: usize, right: usize| -> (usize, usize) {
        let mut l = left as i32;
        let mut r = right as i32;
        while l >= 0 && r < n as i32 && b[l as usize] == b[r as usize] {
            l -= 1;
            r += 1;
        }
        ((l + 1) as usize, r as usize)
    };

    for i in 0..n {
        let (l1, r1) = expand(i, i);
        if r1 - l1 > max_len {
            max_len = r1 - l1;
            start = l1;
        }
        if i + 1 < n {
            let (l2, r2) = expand(i, i + 1);
            if r2 - l2 > max_len {
                max_len = r2 - l2;
                start = l2;
            }
        }
    }

    s[start..start + max_len].to_string()
}

fn main() {
    println!("{}", longest_palindrome("babad".to_string()));
}

#[cfg(test)]
mod tests {
    use super::longest_palindrome;

    #[test]
    fn example_one() {
        let out = longest_palindrome("babad".to_string());
        assert!(out == "bab" || out == "aba");
    }

    #[test]
    fn example_two() {
        assert_eq!(longest_palindrome("cbbd".to_string()), "bb");
    }
}
