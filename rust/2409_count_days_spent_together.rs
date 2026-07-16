/// LeetCode #2409 - Count Days Spent Together
fn day_of_year(date: &str) -> i32 {
    let days = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
    let month: usize = date[0..2].parse().unwrap();
    let day: i32 = date[3..5].parse().unwrap();
    days[..month - 1].iter().sum::<i32>() + day
}

fn count_days_together(
    arrive_alice: String,
    leave_alice: String,
    arrive_bob: String,
    leave_bob: String,
) -> i32 {
    let start = day_of_year(&arrive_alice).max(day_of_year(&arrive_bob));
    let end = day_of_year(&leave_alice).min(day_of_year(&leave_bob));
    (end - start + 1).max(0)
}

fn main() {
    println!(
        "{}",
        count_days_together(
            "08-15".to_string(),
            "08-18".to_string(),
            "08-16".to_string(),
            "08-19".to_string()
        )
    );
}

#[cfg(test)]
mod tests {
    use super::count_days_together;

    #[test]
    fn example_one() {
        assert_eq!(
            count_days_together(
                "08-15".to_string(),
                "08-18".to_string(),
                "08-16".to_string(),
                "08-19".to_string()
            ),
            3
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(
            count_days_together(
                "10-01".to_string(),
                "10-31".to_string(),
                "11-01".to_string(),
                "12-31".to_string()
            ),
            0
        );
    }
}
