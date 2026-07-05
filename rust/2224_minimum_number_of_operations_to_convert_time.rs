/// LeetCode #2224 - Minimum Number of Operations to Convert Time
fn convert_time(current: String, correct: String) -> i32 {
    let to_minutes = |t: &str| {
        let h: i32 = t[..2].parse().unwrap();
        let m: i32 = t[3..].parse().unwrap();
        h * 60 + m
    };

    let mut diff = to_minutes(&correct) - to_minutes(&current);
    let mut ans = 0i32;
    for coin in [60, 15, 5, 1] {
        ans += diff / coin;
        diff %= coin;
    }
    ans
}

fn main() {
    println!("{}", convert_time("02:30".into(), "04:35".into()));
}

#[cfg(test)]
mod tests {
    use super::convert_time;

    #[test]
    fn example_one() {
        assert_eq!(convert_time("02:30".into(), "04:35".into()), 3);
    }

    #[test]
    fn example_two() {
        assert_eq!(convert_time("11:00".into(), "11:01".into()), 1);
    }
}
