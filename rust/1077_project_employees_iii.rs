/// LeetCode #1077 - Project Employees III (SQL; Rust analogue)
use std::collections::HashMap;

fn project_employees_iii(
    project: Vec<(i32, i32)>,
    employee: Vec<(i32, String, i32)>,
) -> Vec<(i32, i32)> {
    let years: HashMap<i32, i32> = employee.into_iter().map(|(id, _, y)| (id, y)).collect();
    let mut by_p: HashMap<i32, Vec<(i32, i32)>> = HashMap::new();
    for (pid, eid) in project {
        by_p.entry(pid).or_default().push((eid, years[&eid]));
    }
    let mut ans = Vec::new();
    for (pid, rows) in by_p {
        let mx = rows.iter().map(|r| r.1).max().unwrap();
        for (eid, y) in rows {
            if y == mx {
                ans.push((pid, eid));
            }
        }
    }
    ans.sort();
    ans
}

fn main() {
    println!("ok");
}

#[cfg(test)]
mod tests {
    use super::project_employees_iii;

    #[test]
    fn example() {
        let project = vec![(1, 1), (1, 2), (1, 3), (2, 1), (2, 4)];
        let employee = vec![
            (1, "Khaled".into(), 3),
            (2, "Ali".into(), 2),
            (3, "John".into(), 3),
            (4, "Doe".into(), 2),
        ];
        assert_eq!(
            project_employees_iii(project, employee),
            vec![(1, 1), (1, 3), (2, 1)]
        );
    }
}
