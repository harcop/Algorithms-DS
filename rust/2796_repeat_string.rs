/// LeetCode #2796 - Repeat String (JS problem; Rust analogue)
fn replicate(s: &str, times: usize) -> String {
    s.repeat(times)
}

fn main() {
    println!("{}", replicate("hello", 2));
}

#[cfg(test)]
mod tests {
    use super::replicate;

    #[test]
    fn example_one() {
        assert_eq!(replicate("hello", 2), "hellohello");
    }

    #[test]
    fn example_two() {
        assert_eq!(replicate("code", 3), "codecodecode");
    }

    #[test]
    fn example_three() {
        assert_eq!(replicate("js", 1), "js");
    }
}
