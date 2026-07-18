/// LeetCode #2468 - Split Message Based on Limit
fn split_message(message: String, limit: i32) -> Vec<String> {
    fn digit_len(num: i32) -> i32 {
        num.to_string().len() as i32
    }

    let n = message.len() as i32;
    let limit = limit;
    let mut parts = 1;
    let mut index_digits = digit_len(1);

    while parts * limit < parts * (digit_len(parts) + 3) + index_digits + n {
        if digit_len(parts) * 2 + 3 >= limit {
            return Vec::new();
        }
        parts += 1;
        index_digits += digit_len(parts);
    }

    let bytes = message.as_bytes();
    let mut answer = Vec::with_capacity(parts as usize);
    let mut offset = 0usize;

    for index in 1..=parts {
        let suffix = format!("<{index}/{parts}>");
        let take = (limit as usize).saturating_sub(suffix.len());
        let end = (offset + take).min(bytes.len());
        let mut part = String::from_utf8_lossy(&bytes[offset..end]).into_owned();
        part.push_str(&suffix);
        answer.push(part);
        offset = end;
    }

    answer
}

fn main() {
    println!("{:?}", split_message("short message".to_string(), 15));
}

#[cfg(test)]
mod tests {
    use super::split_message;

    #[test]
    fn example_one() {
        assert_eq!(
            split_message("this is really a very awesome message".to_string(), 9),
            vec![
                "thi<1/14>",
                "s i<2/14>",
                "s r<3/14>",
                "eal<4/14>",
                "ly <5/14>",
                "a v<6/14>",
                "ery<7/14>",
                " aw<8/14>",
                "eso<9/14>",
                "me<10/14>",
                " m<11/14>",
                "es<12/14>",
                "sa<13/14>",
                "ge<14/14>",
            ]
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(
            split_message("short message".to_string(), 15),
            vec!["short mess<1/2>", "age<2/2>"]
        );
    }

    #[test]
    fn impossible_when_suffix_too_long() {
        assert!(split_message("a".to_string(), 1).is_empty());
    }
}
