/// LeetCode #2486 - Append Characters to String to Make Subsequence
fn append_characters(s: String, t: String) -> i32 {
    let t_bytes = t.as_bytes();
    let mut i = 0usize;

    for c in s.bytes() {
        if c == t_bytes[i] {
            i += 1;
            if i == t_bytes.len() {
                return 0;
            }
        }
    }

    (t_bytes.len() - i) as i32
}

fn main() {
    println!(
        "{}",
        append_characters("coaching".to_string(), "coding".to_string())
    );
}

#[cfg(test)]
mod tests {
    use super::append_characters;

    #[test]
    fn example_one() {
        assert_eq!(
            append_characters("coaching".to_string(), "coding".to_string()),
            4
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(append_characters("abcde".to_string(), "a".to_string()), 0);
    }

    #[test]
    fn example_three() {
        assert_eq!(append_characters("z".to_string(), "abcde".to_string()), 5);
    }
}
