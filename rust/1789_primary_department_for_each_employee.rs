/// LeetCode #1789 - Primary Department for Each Employee (SQL; Rust analogue)
use std::collections::HashMap;

fn primary_department(employee: Vec<(i32, i32, String)>) -> Vec<(i32, i32)> {
    let mut by_emp: HashMap<i32, Vec<(i32, String)>> = HashMap::new();
    for (employee_id, department_id, primary_flag) in employee {
        by_emp
            .entry(employee_id)
            .or_default()
            .push((department_id, primary_flag));
    }
    let mut ans = Vec::new();
    for (employee_id, depts) in by_emp {
        if depts.len() == 1 {
            ans.push((employee_id, depts[0].0));
        } else if let Some((dept, _)) = depts.iter().find(|(_, f)| f == "Y") {
            ans.push((employee_id, *dept));
        }
    }
    ans.sort();
    ans
}

fn main() {
    let employee = vec![
        (1, 1, "N".into()),
        (2, 1, "Y".into()),
        (2, 2, "N".into()),
        (3, 3, "N".into()),
        (4, 2, "N".into()),
        (4, 3, "Y".into()),
        (4, 4, "N".into()),
    ];
    println!("{:?}", primary_department(employee));
}

#[cfg(test)]
mod tests {
    use super::primary_department;

    #[test]
    fn example_one() {
        let employee = vec![
            (1, 1, "N".into()),
            (2, 1, "Y".into()),
            (2, 2, "N".into()),
            (3, 3, "N".into()),
            (4, 2, "N".into()),
            (4, 3, "Y".into()),
            (4, 4, "N".into()),
        ];
        assert_eq!(
            primary_department(employee),
            vec![(1, 1), (2, 1), (3, 3), (4, 3)]
        );
    }
}
