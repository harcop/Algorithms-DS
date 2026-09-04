/// LeetCode #1147 - Longest Chunked Palindrome Decomposition
fn longest_decomposition(text: String) -> i32 {
    let s = text.as_bytes();
    let mut i = 0;
    let mut j = s.len();
    let mut k = 0;
    while i < j {
        let mut found = false;
        for len in 1..=(j - i) / 2 {
            if s[i..i + len] == s[j - len..j] {
                k += 2;
                i += len;
                j -= len;
                found = true;
                break;
            }
        }
        if !found {
            k += 1;
            break;
        }
    }
    k
}

fn main() {
    println!(
        "{}",
        longest_decomposition("ghiabcdefhelloadamhelloabcdefghi".into())
    );
}

#[cfg(test)]
mod tests {
    use super::longest_decomposition;

    #[test]
    fn example_one() {
        assert_eq!(
            longest_decomposition("ghiabcdefhelloadamhelloabcdefghi".into()),
            7
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(longest_decomposition("merchant".into()), 1);
    }

    #[test]
    fn example_three() {
        assert_eq!(longest_decomposition("antaprezatepzapreanta".into()), 11);
    }
}
