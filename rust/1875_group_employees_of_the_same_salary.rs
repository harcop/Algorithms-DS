/// LeetCode #1875 - Group Employees of the Same Salary (SQL; Rust analogue)
use std::collections::HashMap;

fn group_employees_same_salary(
    employees: Vec<(i32, String, i32)>,
) -> Vec<(i32, String, i32, i32)> {
    let mut by_sal: HashMap<i32, i32> = HashMap::new();
    for (_, _, salary) in &employees {
        *by_sal.entry(*salary).or_insert(0) += 1;
    }
    let mut team_sals: Vec<i32> = by_sal
        .into_iter()
        .filter(|(_, c)| *c >= 2)
        .map(|(s, _)| s)
        .collect();
    team_sals.sort();
    let team_id: HashMap<i32, i32> = team_sals
        .into_iter()
        .enumerate()
        .map(|(i, s)| (s, (i + 1) as i32))
        .collect();
    let mut ans: Vec<(i32, String, i32, i32)> = employees
        .into_iter()
        .filter_map(|(id, name, salary)| {
            team_id.get(&salary).map(|&tid| (id, name, salary, tid))
        })
        .collect();
    ans.sort_by_key(|t| (t.3, t.0));
    ans
}

fn main() {
    let employees = vec![
        (2, "Meir".into(), 3000),
        (3, "Michael".into(), 3000),
        (7, "Addilyn".into(), 7400),
        (8, "Juan".into(), 6100),
        (9, "Kannon".into(), 7400),
    ];
    println!("{:?}", group_employees_same_salary(employees));
}

#[cfg(test)]
mod tests {
    use super::group_employees_same_salary;

    #[test]
    fn example_one() {
        let employees = vec![
            (2, "Meir".into(), 3000),
            (3, "Michael".into(), 3000),
            (7, "Addilyn".into(), 7400),
            (8, "Juan".into(), 6100),
            (9, "Kannon".into(), 7400),
        ];
        assert_eq!(
            group_employees_same_salary(employees),
            vec![
                (2, "Meir".into(), 3000, 1),
                (3, "Michael".into(), 3000, 1),
                (7, "Addilyn".into(), 7400, 2),
                (9, "Kannon".into(), 7400, 2),
            ]
        );
    }
}
