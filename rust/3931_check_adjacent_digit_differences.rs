/// LeetCode #3931 - Check Adjacent Digit Differences
fn is_adjacent_diff_at_most_two(s: String) -> bool {
    let b = s.as_bytes();
    b.windows(2).all(|w| (w[0] as i32 - w[1] as i32).abs() <= 2)
}

fn main() {
    println!("{}", is_adjacent_diff_at_most_two("132".into()));
}

#[cfg(test)]
mod tests {
    use super::is_adjacent_diff_at_most_two;

    #[test]
    fn example1() {
        assert!(is_adjacent_diff_at_most_two("132".into()));
    }

    #[test]
    fn example2() {
        assert!(!is_adjacent_diff_at_most_two("129".into()));
    }
}
