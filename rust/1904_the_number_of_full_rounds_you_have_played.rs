/// LeetCode #1904 - The Number of Full Rounds You Have Played
fn number_of_rounds(login_time: String, logout_time: String) -> i32 {
    fn to_minutes(s: &str) -> i32 {
        let bytes = s.as_bytes();
        (bytes[0] - b'0') as i32 * 600
            + (bytes[1] - b'0') as i32 * 60
            + (bytes[3] - b'0') as i32 * 10
            + (bytes[4] - b'0') as i32
    }
    let mut a = to_minutes(&login_time);
    let mut b = to_minutes(&logout_time);
    if a > b {
        b += 1440;
    }
    a = (a + 14) / 15;
    b /= 15;
    (b - a).max(0)
}

fn main() {
    println!("{}", number_of_rounds("09:31".into(), "10:14".into()));
}

#[cfg(test)]
mod tests {
    use super::number_of_rounds;

    #[test]
    fn example_one() {
        assert_eq!(number_of_rounds("09:31".into(), "10:14".into()), 1);
    }

    #[test]
    fn example_two() {
        assert_eq!(number_of_rounds("21:30".into(), "03:00".into()), 22);
    }
}
