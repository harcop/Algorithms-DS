/// LeetCode #1873 - Calculate Special Bonus (SQL; Rust analogue)
fn calculate_special_bonus(employees: Vec<(i32, String, i32)>) -> Vec<(i32, i32)> {
    let mut ans: Vec<(i32, i32)> = employees
        .into_iter()
        .map(|(employee_id, name, salary)| {
            let bonus = if employee_id % 2 == 1 && !name.starts_with('M') {
                salary
            } else {
                0
            };
            (employee_id, bonus)
        })
        .collect();
    ans.sort_by_key(|t| t.0);
    ans
}

fn main() {
    let employees = vec![
        (2, "Meir".into(), 3000),
        (3, "Michael".into(), 3800),
        (7, "Addilyn".into(), 7400),
        (8, "Juan".into(), 6100),
        (9, "Kannon".into(), 7700),
    ];
    println!("{:?}", calculate_special_bonus(employees));
}

#[cfg(test)]
mod tests {
    use super::calculate_special_bonus;

    #[test]
    fn example_one() {
        let employees = vec![
            (2, "Meir".into(), 3000),
            (3, "Michael".into(), 3800),
            (7, "Addilyn".into(), 7400),
            (8, "Juan".into(), 6100),
            (9, "Kannon".into(), 7700),
        ];
        assert_eq!(
            calculate_special_bonus(employees),
            vec![(2, 0), (3, 0), (7, 7400), (8, 0), (9, 7700)]
        );
    }
}
