/// LeetCode #3110 - Score of a String
fn score_of_string(s: String) -> i32 {
    s.as_bytes()
        .windows(2)
        .map(|w| (w[0] as i32 - w[1] as i32).abs())
        .sum()
}

fn main() {
    println!("{}", score_of_string("hello".into()));
}

#[cfg(test)]
mod tests {
    use super::score_of_string;

    #[test]
    fn example1() {
        assert_eq!(score_of_string("hello".into()), 13);
    }

    #[test]
    fn example2() {
        assert_eq!(score_of_string("zaz".into()), 50);
    }
}
