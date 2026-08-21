/// LeetCode #3338 - Second Highest Salary II (SQL; Rust analogue)
/// employees: (emp_id, salary, dept)
fn find_second_highest_salary(employees: Vec<(i32, i32, String)>) -> Vec<(i32, String)> {
    use std::collections::HashMap;
    let mut by_dept: HashMap<String, Vec<(i32, i32)>> = HashMap::new();
    for (id, salary, dept) in employees {
        by_dept.entry(dept).or_default().push((id, salary));
    }
    let mut ans = Vec::new();
    for (dept, rows) in by_dept {
        let mut salaries: Vec<i32> = rows.iter().map(|r| r.1).collect();
        salaries.sort_unstable_by(|a, b| b.cmp(a));
        salaries.dedup();
        if salaries.len() < 2 {
            continue;
        }
        let second = salaries[1];
        for (id, salary) in rows {
            if salary == second {
                ans.push((id, dept.clone()));
            }
        }
    }
    ans.sort_by_key(|r| r.0);
    ans
}

fn main() {
    let employees = vec![
        (1, 70000, "Sales".into()),
        (2, 80000, "Sales".into()),
    ];
    println!("{:?}", find_second_highest_salary(employees));
}

#[cfg(test)]
mod tests {
    use super::find_second_highest_salary;

    #[test]
    fn example() {
        let employees = vec![
            (1, 70000, "Sales".into()),
            (2, 80000, "Sales".into()),
            (3, 80000, "Sales".into()),
            (4, 90000, "Sales".into()),
            (5, 55000, "IT".into()),
            (6, 65000, "IT".into()),
            (7, 65000, "IT".into()),
            (8, 50000, "Marketing".into()),
            (9, 55000, "Marketing".into()),
            (10, 55000, "HR".into()),
        ];
        assert_eq!(
            find_second_highest_salary(employees),
            vec![
                (2, "Sales".into()),
                (3, "Sales".into()),
                (5, "IT".into()),
                (8, "Marketing".into()),
            ]
        );
    }
}
