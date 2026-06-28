/// LeetCode #2131 - Longest Palindrome by Concatenating Two Letter Words
fn longest_palindrome(words: Vec<String>) -> i32 {
    let mut counts = [[0i32; 26]; 26];
    let mut length = 0;

    for word in words {
        let bytes = word.as_bytes();
        let a = (bytes[0] - b'a') as usize;
        let b = (bytes[1] - b'a') as usize;

        if counts[b][a] > 0 {
            counts[b][a] -= 1;
            length += 4;
        } else {
            counts[a][b] += 1;
        }
    }

    for i in 0..26 {
        if counts[i][i] > 0 {
            return length + 2;
        }
    }

    length
}

fn main() {
    println!(
        "{}",
        longest_palindrome(vec!["lc".into(), "cl".into(), "gg".into()])
    );
}

#[cfg(test)]
mod tests {
    use super::longest_palindrome;

    #[test]
    fn example_one() {
        assert_eq!(
            longest_palindrome(vec!["lc".into(), "cl".into(), "gg".into()]),
            6
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(
            longest_palindrome(vec![
                "ab".into(),
                "ty".into(),
                "yt".into(),
                "lc".into(),
                "cl".into(),
                "ab".into()
            ]),
            8
        );
    }

    #[test]
    fn example_three() {
        assert_eq!(
            longest_palindrome(vec!["cc".into(), "ll".into(), "xx".into()]),
            2
        );
    }
}
