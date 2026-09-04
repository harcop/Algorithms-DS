/// LeetCode #1731 - The Number of Employees Which Report to Each Employee (SQL; Rust analogue)
use std::collections::HashMap;

fn employees_with_reports(
    employees: Vec<(i32, String, Option<i32>, i32)>,
) -> Vec<(i32, String, i32, i32)> {
    let names: HashMap<i32, String> = employees.iter().map(|(id, n, _, _)| (*id, n.clone())).collect();
    let mut reports: HashMap<i32, Vec<i32>> = HashMap::new();
    for (_, _, mgr, age) in &employees {
        if let Some(m) = mgr {
            reports.entry(*m).or_default().push(*age);
        }
    }
    let mut ans: Vec<(i32, String, i32, i32)> = reports
        .into_iter()
        .map(|(id, ages)| {
            let n = ages.len() as i32;
            let avg = (ages.iter().sum::<i32>() as f64 / n as f64).round() as i32;
            (id, names[&id].clone(), n, avg)
        })
        .collect();
    ans.sort_by_key(|r| r.0);
    ans
}

fn main() {
    println!("{:?}", employees_with_reports(vec![]));
}

#[cfg(test)]
mod tests {
    use super::employees_with_reports;

    #[test]
    fn example() {
        let employees = vec![
            (9, "Hercy".into(), None, 43),
            (6, "Alice".into(), Some(9), 41),
            (4, "Bob".into(), Some(9), 36),
            (2, "Winston".into(), None, 37),
        ];
        assert_eq!(
            employees_with_reports(employees),
            vec![(9, "Hercy".into(), 2, 39)]
        );
    }
}
