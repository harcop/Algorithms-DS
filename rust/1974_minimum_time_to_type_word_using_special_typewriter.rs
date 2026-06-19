/// LeetCode #1974 - Minimum Time to Type Word Using Special Typewriter
fn min_time_to_type(word: String) -> i32 {
    let mut ans = word.len() as i32;
    let mut prev = b'a';
    for b in word.bytes() {
        let d = (b as i32 - prev as i32).abs();
        ans += d.min(26 - d);
        prev = b;
    }
    ans
}

fn main() {
    println!("{}", min_time_to_type("abc".into()));
}

#[cfg(test)]
mod tests {
    use super::min_time_to_type;

    #[test]
    fn example_one() {
        assert_eq!(min_time_to_type("abc".into()), 5);
    }

    #[test]
    fn example_two() {
        assert_eq!(min_time_to_type("bza".into()), 7);
    }

    #[test]
    fn example_three() {
        assert_eq!(min_time_to_type("zjpc".into()), 34);
    }
}
