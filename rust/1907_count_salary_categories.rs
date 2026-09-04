/// LeetCode #1907 - Count Salary Categories (SQL; Rust analogue)
fn count_salary_categories(accounts: Vec<(i32, i32)>) -> Vec<(String, i32)> {
    let mut low = 0;
    let mut average = 0;
    let mut high = 0;
    for (_, income) in accounts {
        if income < 20000 {
            low += 1;
        } else if income <= 50000 {
            average += 1;
        } else {
            high += 1;
        }
    }
    vec![
        ("Low Salary".into(), low),
        ("Average Salary".into(), average),
        ("High Salary".into(), high),
    ]
}

fn main() {
    let accounts = vec![(3, 108939), (2, 12747), (8, 87709), (6, 91796)];
    println!("{:?}", count_salary_categories(accounts));
}

#[cfg(test)]
mod tests {
    use super::count_salary_categories;

    #[test]
    fn example_one() {
        let accounts = vec![(3, 108939), (2, 12747), (8, 87709), (6, 91796)];
        assert_eq!(
            count_salary_categories(accounts),
            vec![
                ("Low Salary".into(), 1),
                ("Average Salary".into(), 0),
                ("High Salary".into(), 3),
            ]
        );
    }
}
