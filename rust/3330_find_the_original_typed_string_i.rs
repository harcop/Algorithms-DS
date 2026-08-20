/// LeetCode #3330 - Find the Original Typed String I
fn possible_string_count(word: String) -> i32 {
    let b = word.as_bytes();
    1 + b.windows(2).filter(|w| w[0] == w[1]).count() as i32
}

fn main() {
    println!("{}", possible_string_count("abbcccc".into()));
}

#[cfg(test)]
mod tests {
    use super::possible_string_count;

    #[test]
    fn example1() {
        assert_eq!(possible_string_count("abbcccc".into()), 5);
    }

    #[test]
    fn example2() {
        assert_eq!(possible_string_count("abcd".into()), 1);
    }

    #[test]
    fn example3() {
        assert_eq!(possible_string_count("aaaa".into()), 4);
    }
}
