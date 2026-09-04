/// LeetCode #569 - Median Employee Salary (SQL; Rust analogue)
use std::collections::HashMap;

fn median_employee_salary(employee: Vec<(i32, String, i32)>) -> Vec<(i32, String, i32)> {
    let mut by_co: HashMap<String, Vec<(i32, i32)>> = HashMap::new();
    for (id, company, salary) in employee {
        by_co.entry(company).or_default().push((salary, id));
    }
    let mut ans = Vec::new();
    for (company, mut rows) in by_co {
        rows.sort();
        let n = rows.len();
        let lo = (n as f64 / 2.0).ceil() as usize - 1;
        let hi = n / 2;
        for i in lo..=hi {
            ans.push((rows[i].1, company.clone(), rows[i].0));
        }
    }
    ans.sort_by(|a, b| a.1.cmp(&b.1).then(a.2.cmp(&b.2)).then(a.0.cmp(&b.0)));
    ans
}

fn main() {
    println!("ok");
}

#[cfg(test)]
mod tests {
    use super::median_employee_salary;

    #[test]
    fn example() {
        let employee = vec![
            (1, "A".into(), 2341),
            (2, "A".into(), 341),
            (3, "A".into(), 15),
            (4, "A".into(), 15314),
            (5, "A".into(), 451),
            (6, "A".into(), 513),
            (7, "B".into(), 15),
            (8, "B".into(), 13),
            (9, "B".into(), 1154),
            (10, "B".into(), 1345),
            (11, "B".into(), 1221),
            (12, "B".into(), 234),
            (13, "C".into(), 2345),
            (14, "C".into(), 2645),
            (15, "C".into(), 2645),
            (16, "C".into(), 2652),
            (17, "C".into(), 65),
        ];
        assert_eq!(
            median_employee_salary(employee),
            vec![
                (5, "A".into(), 451),
                (6, "A".into(), 513),
                (12, "B".into(), 234),
                (9, "B".into(), 1154),
                (14, "C".into(), 2645),
            ]
        );
    }
}
