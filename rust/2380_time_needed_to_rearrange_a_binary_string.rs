/// LeetCode #2380 - Time Needed to Rearrange a Binary String
fn seconds_to_remove_occurrences(s: String) -> i32 {
    let mut ans = 0;
    let mut zeros = 0;

    for c in s.bytes() {
        if c == b'0' {
            zeros += 1;
        } else if zeros > 0 {
            ans = (ans + 1).max(zeros);
        }
    }

    ans
}

fn main() {
    println!("{}", seconds_to_remove_occurrences("0110101".to_string()));
}

#[cfg(test)]
mod tests {
    use super::seconds_to_remove_occurrences;

    #[test]
    fn example_one() {
        assert_eq!(seconds_to_remove_occurrences("0110101".to_string()), 4);
    }

    #[test]
    fn example_two() {
        assert_eq!(seconds_to_remove_occurrences("11100".to_string()), 0);
    }
}
