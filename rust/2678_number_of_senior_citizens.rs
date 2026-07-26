/// LeetCode #2678 - Number of Senior Citizens
fn count_seniors(details: Vec<String>) -> i32 {
    details
        .iter()
        .filter(|s| s[11..13].parse::<i32>().unwrap_or(0) > 60)
        .count() as i32
}

fn main() {
    println!(
        "{}",
        count_seniors(vec![
            "7868190130M7522".into(),
            "5303914400F9211".into(),
            "9273338290F4010".into(),
        ])
    );
}

#[cfg(test)]
mod tests {
    use super::count_seniors;

    #[test]
    fn example_one() {
        assert_eq!(
            count_seniors(vec![
                "7868190130M7522".into(),
                "5303914400F9211".into(),
                "9273338290F4010".into(),
            ]),
            2
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(
            count_seniors(vec!["1313579440F2036".into(), "2921522980M5644".into()]),
            0
        );
    }
}
