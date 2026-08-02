/// LeetCode #2914 - Minimum Number of Changes to Make Binary String Beautiful
fn min_changes(s: String) -> i32 {
    let bytes = s.as_bytes();
    (1..bytes.len())
        .step_by(2)
        .filter(|&i| bytes[i] != bytes[i - 1])
        .count() as i32
}

fn main() {
    println!("{}", min_changes("1001".into()));
}

#[cfg(test)]
mod tests {
    use super::min_changes;

    #[test]
    fn example_one() {
        assert_eq!(min_changes("1001".into()), 2);
    }

    #[test]
    fn example_two() {
        assert_eq!(min_changes("10".into()), 1);
    }

    #[test]
    fn example_three() {
        assert_eq!(min_changes("0000".into()), 0);
    }
}
