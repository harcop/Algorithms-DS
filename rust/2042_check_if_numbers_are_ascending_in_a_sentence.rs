/// LeetCode #2042 - Check if Numbers Are Ascending in a Sentence
fn are_numbers_ascending(s: String) -> bool {
    let mut pre = 0i32;
    for t in s.split_whitespace() {
        if t.as_bytes()[0].is_ascii_digit() {
            let cur: i32 = t.parse().unwrap();
            if cur <= pre {
                return false;
            }
            pre = cur;
        }
    }
    true
}

fn main() {
    println!(
        "{}",
        are_numbers_ascending("1 box has 3 blue 4 red 6 green and 12 yellow marbles".into())
    );
}

#[cfg(test)]
mod tests {
    use super::are_numbers_ascending;

    #[test]
    fn example_one() {
        assert!(are_numbers_ascending(
            "1 box has 3 blue 4 red 6 green and 12 yellow marbles".into()
        ));
    }

    #[test]
    fn example_two() {
        assert!(!are_numbers_ascending("hello world 5 x 5".into()));
    }

    #[test]
    fn example_three() {
        assert!(!are_numbers_ascending(
            "sunset is at 7 51 pm overnight lows will be in the low 50 and 60 s".into()
        ));
    }
}
