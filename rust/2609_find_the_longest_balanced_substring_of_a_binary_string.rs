/// LeetCode #2609 - Find the Longest Balanced Substring of a Binary String
fn find_the_longest_balanced_substring(s: String) -> i32 {
    let mut zero = 0;
    let mut one = 0;
    let mut ans = 0;
    for c in s.bytes() {
        if c == b'0' {
            if one > 0 {
                zero = 0;
                one = 0;
            }
            zero += 1;
        } else {
            one += 1;
            ans = ans.max(2 * zero.min(one));
        }
    }
    ans
}

fn main() {
    println!(
        "{}",
        find_the_longest_balanced_substring("01000111".to_string())
    );
}

#[cfg(test)]
mod tests {
    use super::find_the_longest_balanced_substring;

    #[test]
    fn example_one() {
        assert_eq!(
            find_the_longest_balanced_substring("01000111".to_string()),
            6
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(
            find_the_longest_balanced_substring("00111".to_string()),
            4
        );
    }

    #[test]
    fn example_three() {
        assert_eq!(find_the_longest_balanced_substring("111".to_string()), 0);
    }
}
