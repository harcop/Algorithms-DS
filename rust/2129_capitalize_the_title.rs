/// LeetCode #2129 - Capitalize the Title
fn capitalize_title(title: String) -> String {
    title
        .split_whitespace()
        .map(|word| {
            let word = word.to_ascii_lowercase();
            if word.len() <= 2 {
                word
            } else {
                let mut bytes = word.into_bytes();
                bytes[0] = bytes[0].to_ascii_uppercase();
                String::from_utf8(bytes).unwrap()
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}

fn main() {
    println!("{}", capitalize_title("capiTalIze tHe titLe".into()));
}

#[cfg(test)]
mod tests {
    use super::capitalize_title;

    #[test]
    fn example_one() {
        assert_eq!(
            capitalize_title("capiTalIze tHe titLe".into()),
            "Capitalize The Title"
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(
            capitalize_title("First leTTeR of EACH Word".into()),
            "First Letter of Each Word"
        );
    }

    #[test]
    fn example_three() {
        assert_eq!(
            capitalize_title("i lOve leetcode".into()),
            "i Love Leetcode"
        );
    }
}
