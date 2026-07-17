/// LeetCode #2437 - Number of Valid Clock Times
fn count_time(time: String) -> i32 {
    fn matches(pattern: &[u8], value: i32) -> bool {
        let formatted = format!("{value:02}");
        pattern
            .iter()
            .zip(formatted.bytes())
            .all(|(&expected, actual)| expected == b'?' || expected == actual)
    }

    let bytes = time.as_bytes();
    let mut answer = 0;

    for hour in 0..24 {
        for minute in 0..60 {
            if matches(&bytes[..2], hour) && matches(&bytes[3..], minute) {
                answer += 1;
            }
        }
    }

    answer
}

fn main() {
    println!("{}", count_time("?5:00".to_string()));
}

#[cfg(test)]
mod tests {
    use super::count_time;

    #[test]
    fn example_one() {
        assert_eq!(count_time("?5:00".to_string()), 2);
    }

    #[test]
    fn example_two() {
        assert_eq!(count_time("0?:0?".to_string()), 100);
    }

    #[test]
    fn all_unknown() {
        assert_eq!(count_time("??:??".to_string()), 1440);
    }
}
