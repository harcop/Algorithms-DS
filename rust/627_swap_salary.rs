/// LeetCode #627 - Swap Salary (SQL; Rust analogue)
fn swap_salary(mut salary: Vec<(i32, String, char, i32)>) -> Vec<(i32, String, char, i32)> {
    for row in &mut salary {
        row.2 = if row.2 == 'm' { 'f' } else { 'm' };
    }
    salary
}

fn main() {
    println!("ok");
}

#[cfg(test)]
mod tests {
    use super::swap_salary;

    #[test]
    fn example() {
        let salary = vec![
            (1, "A".into(), 'm', 2500),
            (2, "B".into(), 'f', 1500),
            (3, "C".into(), 'm', 5500),
            (4, "D".into(), 'f', 500),
        ];
        assert_eq!(
            swap_salary(salary),
            vec![
                (1, "A".into(), 'f', 2500),
                (2, "B".into(), 'm', 1500),
                (3, "C".into(), 'f', 5500),
                (4, "D".into(), 'm', 500),
            ]
        );
    }
}
