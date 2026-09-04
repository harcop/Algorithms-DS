/// LeetCode #570 - Managers with at Least 5 Direct Reports (SQL; Rust analogue)
use std::collections::HashMap;

fn managers_with_at_least_5(
    employee: Vec<(i32, String, String, Option<i32>)>,
) -> Vec<String> {
    let mut names: HashMap<i32, String> = HashMap::new();
    let mut reports: HashMap<i32, i32> = HashMap::new();
    for (id, name, _, manager) in employee {
        names.insert(id, name);
        if let Some(m) = manager {
            *reports.entry(m).or_insert(0) += 1;
        }
    }
    let mut ans: Vec<String> = reports
        .into_iter()
        .filter(|(_, c)| *c >= 5)
        .filter_map(|(id, _)| names.get(&id).cloned())
        .collect();
    ans.sort();
    ans
}

fn main() {
    println!("ok");
}

#[cfg(test)]
mod tests {
    use super::managers_with_at_least_5;

    #[test]
    fn example() {
        let employee = vec![
            (101, "John".into(), "A".into(), None),
            (102, "Dan".into(), "A".into(), Some(101)),
            (103, "James".into(), "A".into(), Some(101)),
            (104, "Amy".into(), "A".into(), Some(101)),
            (105, "Anne".into(), "A".into(), Some(101)),
            (106, "Ron".into(), "B".into(), Some(101)),
        ];
        assert_eq!(managers_with_at_least_5(employee), vec!["John".to_string()]);
    }
}
