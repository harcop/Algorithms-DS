/// LeetCode #3057 - Employees Project Allocation (SQL; Rust analogue)
use std::collections::HashMap;

fn employees_project_allocation(
    project: Vec<(i32, i32, i32)>,
    employees: Vec<(i32, String, String)>,
) -> Vec<(i32, i32, String, i32)> {
    let emp: HashMap<i32, (String, String)> = employees
        .into_iter()
        .map(|(id, name, team)| (id, (name, team)))
        .collect();

    let mut team_sum: HashMap<String, f64> = HashMap::new();
    let mut team_cnt: HashMap<String, u32> = HashMap::new();

    for (_, employee_id, workload) in &project {
        if let Some((_, team)) = emp.get(employee_id) {
            *team_sum.entry(team.clone()).or_default() += *workload as f64;
            *team_cnt.entry(team.clone()).or_default() += 1;
        }
    }

    let team_avg: HashMap<String, f64> = team_sum
        .into_iter()
        .map(|(team, sum)| {
            let cnt = team_cnt[&team] as f64;
            (team, sum / cnt)
        })
        .collect();

    let mut ans: Vec<_> = project
        .into_iter()
        .filter_map(|(project_id, employee_id, workload)| {
            let (name, team) = emp.get(&employee_id)?;
            let avg = team_avg[team];
            if (workload as f64) > avg {
                Some((employee_id, project_id, name.clone(), workload))
            } else {
                None
            }
        })
        .collect();
    ans.sort_by(|a, b| a.0.cmp(&b.0).then(a.1.cmp(&b.1)));
    ans
}

fn main() {
    let project = vec![(1, 1, 45), (1, 2, 90), (2, 3, 12), (2, 4, 68)];
    let employees = vec![
        (1, "Khaled".into(), "A".into()),
        (2, "Ali".into(), "B".into()),
        (3, "John".into(), "B".into()),
        (4, "Doe".into(), "A".into()),
    ];
    println!("{:?}", employees_project_allocation(project, employees));
}

#[cfg(test)]
mod tests {
    use super::employees_project_allocation;

    #[test]
    fn example() {
        let project = vec![(1, 1, 45), (1, 2, 90), (2, 3, 12), (2, 4, 68)];
        let employees = vec![
            (1, "Khaled".into(), "A".into()),
            (2, "Ali".into(), "B".into()),
            (3, "John".into(), "B".into()),
            (4, "Doe".into(), "A".into()),
        ];
        assert_eq!(
            employees_project_allocation(project, employees),
            vec![(2, 1, "Ali".into(), 90), (4, 2, "Doe".into(), 68)]
        );
    }
}
