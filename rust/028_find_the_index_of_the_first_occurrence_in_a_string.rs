/// LeetCode #28 - Find the Index of the First Occurrence in a String
fn str_str(haystack: String, needle: String) -> i32 {
    haystack.find(&needle).map_or(-1, |idx| idx as i32)
}

fn main() {
    println!("{}", str_str("sadbutsad".to_string(), "sad".to_string()));
}

#[cfg(test)]
mod tests {
    use super::str_str;

    #[test]
    fn example_one() {
        assert_eq!(str_str("sadbutsad".to_string(), "sad".to_string()), 0);
    }

    #[test]
    fn example_two() {
        assert_eq!(str_str("leetcode".to_string(), "leeto".to_string()), -1);
    }
}
