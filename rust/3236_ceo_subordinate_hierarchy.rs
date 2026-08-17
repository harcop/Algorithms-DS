/// LeetCode #3236 - CEO Subordinate Hierarchy (SQL; Rust analogue)
/// employees: (employee_id, employee_name, manager_id, salary)
fn ceo_subordinate_hierarchy(
    employees: Vec<(i32, String, Option<i32>, i32)>,
) -> Vec<(i32, String, i32, i32)> {
    use std::collections::{HashMap, VecDeque};
    let ceo = employees
        .iter()
        .find(|e| e.2.is_none())
        .expect("CEO");
    let ceo_id = ceo.0;
    let ceo_salary = ceo.3;
    let mut children: HashMap<i32, Vec<(i32, String, i32)>> = HashMap::new();
    for (id, name, manager, salary) in &employees {
        if let Some(m) = manager {
            children
                .entry(*m)
                .or_default()
                .push((*id, name.clone(), *salary));
        }
    }
    let mut ans = Vec::new();
    let mut q = VecDeque::new();
    q.push_back((ceo_id, 0));
    while let Some((id, level)) = q.pop_front() {
        if let Some(kids) = children.get(&id) {
            for (cid, name, salary) in kids {
                ans.push((*cid, name.clone(), level + 1, salary - ceo_salary));
                q.push_back((*cid, level + 1));
            }
        }
    }
    ans.sort_by(|a, b| a.2.cmp(&b.2).then(a.0.cmp(&b.0)));
    ans
}

fn main() {
    let employees = vec![
        (1, "Alice".into(), None, 150000),
        (2, "Bob".into(), Some(1), 120000),
        (3, "Charlie".into(), Some(1), 110000),
    ];
    println!("{:?}", ceo_subordinate_hierarchy(employees));
}

#[cfg(test)]
mod tests {
    use super::ceo_subordinate_hierarchy;

    #[test]
    fn example() {
        let employees = vec![
            (1, "Alice".into(), None, 150000),
            (2, "Bob".into(), Some(1), 120000),
            (3, "Charlie".into(), Some(1), 110000),
            (4, "David".into(), Some(2), 105000),
            (5, "Eve".into(), Some(2), 100000),
            (6, "Frank".into(), Some(3), 95000),
            (7, "Grace".into(), Some(3), 98000),
            (8, "Helen".into(), Some(5), 90000),
        ];
        assert_eq!(
            ceo_subordinate_hierarchy(employees),
            vec![
                (2, "Bob".into(), 1, -30000),
                (3, "Charlie".into(), 1, -40000),
                (4, "David".into(), 2, -45000),
                (5, "Eve".into(), 2, -50000),
                (6, "Frank".into(), 2, -55000),
                (7, "Grace".into(), 2, -52000),
                (8, "Helen".into(), 3, -60000),
            ]
        );
    }
}
