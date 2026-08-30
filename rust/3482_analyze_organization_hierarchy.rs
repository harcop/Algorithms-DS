/// LeetCode #3482 - Analyze Organization Hierarchy (SQL; Rust analogue)
use std::collections::HashMap;

fn analyze_organization_hierarchy(
    employees: Vec<(i32, String, Option<i32>, i32, String)>,
) -> Vec<(i32, String, i32, i32, i32)> {
    let n = employees.len();
    let mut id_to_idx = HashMap::new();
    for (i, e) in employees.iter().enumerate() {
        id_to_idx.insert(e.0, i);
    }
    let mut children = vec![Vec::new(); n];
    let mut ceo = 0;
    for (i, e) in employees.iter().enumerate() {
        match e.2 {
            Some(mid) => children[id_to_idx[&mid]].push(i),
            None => ceo = i,
        }
    }
    let mut level = vec![0i32; n];
    let mut team_size = vec![0i32; n];
    let mut budget = vec![0i32; n];
    fn dfs(
        u: usize,
        lv: i32,
        children: &[Vec<usize>],
        employees: &[(i32, String, Option<i32>, i32, String)],
        level: &mut [i32],
        team_size: &mut [i32],
        budget: &mut [i32],
    ) {
        level[u] = lv;
        budget[u] = employees[u].3;
        team_size[u] = 0;
        for &v in &children[u] {
            dfs(v, lv + 1, children, employees, level, team_size, budget);
            team_size[u] += team_size[v] + 1;
            budget[u] += budget[v];
        }
    }
    dfs(
        ceo,
        1,
        &children,
        &employees,
        &mut level,
        &mut team_size,
        &mut budget,
    );
    let mut order: Vec<usize> = (0..n).collect();
    order.sort_by(|&a, &b| {
        level[a]
            .cmp(&level[b])
            .then(budget[b].cmp(&budget[a]))
            .then(employees[a].1.cmp(&employees[b].1))
    });
    order
        .into_iter()
        .map(|i| {
            (
                employees[i].0,
                employees[i].1.clone(),
                level[i],
                team_size[i],
                budget[i],
            )
        })
        .collect()
}

fn main() {
    let employees = vec![
        (1, "Alice".into(), None, 12000, "Executive".into()),
        (2, "Bob".into(), Some(1), 10000, "Sales".into()),
    ];
    println!("{:?}", analyze_organization_hierarchy(employees));
}

#[cfg(test)]
mod tests {
    use super::analyze_organization_hierarchy;

    #[test]
    fn example() {
        let employees = vec![
            (1, "Alice".into(), None, 12000, "Executive".into()),
            (2, "Bob".into(), Some(1), 10000, "Sales".into()),
            (3, "Charlie".into(), Some(1), 10000, "Engineering".into()),
            (4, "David".into(), Some(2), 7500, "Sales".into()),
            (5, "Eva".into(), Some(2), 7500, "Sales".into()),
            (6, "Frank".into(), Some(3), 9000, "Engineering".into()),
            (7, "Grace".into(), Some(3), 8500, "Engineering".into()),
            (8, "Hank".into(), Some(4), 6000, "Sales".into()),
            (9, "Ivy".into(), Some(6), 7000, "Engineering".into()),
            (10, "Judy".into(), Some(6), 7000, "Engineering".into()),
        ];
        assert_eq!(
            analyze_organization_hierarchy(employees),
            vec![
                (1, "Alice".into(), 1, 9, 84500),
                (3, "Charlie".into(), 2, 4, 41500),
                (2, "Bob".into(), 2, 3, 31000),
                (6, "Frank".into(), 3, 2, 23000),
                (4, "David".into(), 3, 1, 13500),
                (7, "Grace".into(), 3, 0, 8500),
                (5, "Eva".into(), 3, 0, 7500),
                (9, "Ivy".into(), 4, 0, 7000),
                (10, "Judy".into(), 4, 0, 7000),
                (8, "Hank".into(), 4, 0, 6000),
            ]
        );
    }
}
