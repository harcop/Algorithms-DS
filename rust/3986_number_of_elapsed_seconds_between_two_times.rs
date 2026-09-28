/// LeetCode #3986 - Number of Elapsed Seconds Between Two Times
fn seconds_between(start_time: String, end_time: String) -> i32 {
    fn to_seconds(s: &str) -> i32 {
        let h: i32 = s[0..2].parse().unwrap();
        let m: i32 = s[3..5].parse().unwrap();
        let sec: i32 = s[6..8].parse().unwrap();
        h * 3600 + m * 60 + sec
    }
    to_seconds(&end_time) - to_seconds(&start_time)
}

fn main() {
    println!(
        "{}",
        seconds_between("01:00:00".into(), "01:00:25".into())
    );
}

#[cfg(test)]
mod tests {
    use super::seconds_between;

    #[test]
    fn example1() {
        assert_eq!(seconds_between("01:00:00".into(), "01:00:25".into()), 25);
    }

    #[test]
    fn example2() {
        assert_eq!(seconds_between("12:34:56".into(), "13:00:00".into()), 1504);
    }
}
