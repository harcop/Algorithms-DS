/// LeetCode #1075 - Project Employees I (SQL; Rust analogue)

fn round2(x: f64) -> f64 {
    (x * 100.0).round() / 100.0
}

use std::collections::HashMap;

fn project_employees_i(
    project: Vec<(i32, i32)>,
    employee: Vec<(i32, String, i32)>,
) -> Vec<(i32, f64)> {
    let years: HashMap<i32, i32> = employee.into_iter().map(|(id, _, y)| (id, y)).collect();
    let mut acc: HashMap<i32, (i32, i32)> = HashMap::new();
    for (pid, eid) in project {
        let e = acc.entry(pid).or_insert((0, 0));
        e.0 += years[&eid];
        e.1 += 1;
    }
    let mut ans: Vec<(i32, f64)> = acc
        .into_iter()
        .map(|(pid, (s, n))| (pid, round2(s as f64 / n as f64)))
        .collect();
    ans.sort_by_key(|r| r.0);
    ans
}

fn main() {
    println!("ok");
}

#[cfg(test)]
mod tests {
    use super::project_employees_i;

    #[test]
    fn example() {
        let project = vec![(1, 1), (1, 2), (1, 3), (2, 1), (2, 4)];
        let employee = vec![
            (1, "Khaled".into(), 3),
            (2, "Ali".into(), 2),
            (3, "John".into(), 1),
            (4, "Doe".into(), 2),
        ];
        let got = project_employees_i(project, employee);
        assert_eq!(got[0].0, 1);
        assert!((got[0].1 - 2.0).abs() < 1e-9);
        assert_eq!(got[1].0, 2);
        assert!((got[1].1 - 2.5).abs() < 1e-9);
    }
}
