/// LeetCode #3580 - Find Consistently Improving Employees (SQL; Rust analogue)
use std::collections::HashMap;

fn find_consistently_improving_employees(
    employees: Vec<(i32, String)>,
    reviews: Vec<(i32, i32, String, i32)>,
) -> Vec<(i32, String, i32)> {
    let names: HashMap<i32, String> = employees.into_iter().collect();
    let mut by_emp: HashMap<i32, Vec<(String, i32)>> = HashMap::new();
    for (_rid, eid, date, rating) in reviews {
        by_emp.entry(eid).or_default().push((date, rating));
    }
    let mut ans = Vec::new();
    for (eid, mut recs) in by_emp {
        recs.sort_by(|a, b| a.0.cmp(&b.0));
        if recs.len() < 3 {
            continue;
        }
        let n = recs.len();
        let a = recs[n - 3].1;
        let b = recs[n - 2].1;
        let c = recs[n - 1].1;
        if a < b && b < c {
            if let Some(name) = names.get(&eid) {
                ans.push((eid, name.clone(), c - a));
            }
        }
    }
    ans.sort_by(|x, y| y.2.cmp(&x.2).then(x.1.cmp(&y.1)));
    ans
}

fn main() {
    println!("{:?}", find_consistently_improving_employees(vec![], vec![]));
}

#[cfg(test)]
mod tests {
    use super::find_consistently_improving_employees;

    #[test]
    fn example() {
        let employees = vec![
            (1, "Alice Johnson".into()),
            (2, "Bob Smith".into()),
            (3, "Carol Davis".into()),
            (4, "David Wilson".into()),
            (5, "Emma Brown".into()),
        ];
        let reviews = vec![
            (1, 1, "2023-01-15".into(), 2),
            (2, 1, "2023-04-15".into(), 3),
            (3, 1, "2023-07-15".into(), 4),
            (4, 1, "2023-10-15".into(), 5),
            (5, 2, "2023-02-01".into(), 3),
            (6, 2, "2023-05-01".into(), 2),
            (7, 2, "2023-08-01".into(), 4),
            (8, 2, "2023-11-01".into(), 5),
            (9, 3, "2023-03-10".into(), 1),
            (10, 3, "2023-06-10".into(), 2),
            (11, 3, "2023-09-10".into(), 3),
            (12, 3, "2023-12-10".into(), 4),
            (13, 4, "2023-01-20".into(), 4),
            (14, 4, "2023-04-20".into(), 4),
            (15, 4, "2023-07-20".into(), 4),
            (16, 5, "2023-02-15".into(), 3),
            (17, 5, "2023-05-15".into(), 2),
        ];
        assert_eq!(
            find_consistently_improving_employees(employees, reviews),
            vec![
                (2, "Bob Smith".into(), 3),
                (1, "Alice Johnson".into(), 2),
                (3, "Carol Davis".into(), 2),
            ]
        );
    }
}
