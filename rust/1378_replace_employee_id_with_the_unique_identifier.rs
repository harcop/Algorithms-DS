/// LeetCode #1378 - Replace Employee ID With The Unique Identifier (SQL; Rust analogue)
use std::collections::HashMap;

fn replace_employee_id(
    employees: Vec<(i32, String)>,
    uni: Vec<(i32, i32)>,
) -> Vec<(Option<i32>, String)> {
    let map: HashMap<i32, i32> = uni.into_iter().collect();
    employees
        .into_iter()
        .map(|(id, name)| (map.get(&id).copied(), name))
        .collect()
}

fn main() {
    println!("{:?}", replace_employee_id(vec![], vec![]));
}

#[cfg(test)]
mod tests {
    use super::replace_employee_id;

    #[test]
    fn example() {
        let employees = vec![
            (1, "Alice".into()),
            (7, "Bob".into()),
            (11, "Meir".into()),
            (90, "Winston".into()),
            (3, "Jonathan".into()),
        ];
        let uni = vec![(3, 1), (11, 2), (90, 3)];
        let mut got = replace_employee_id(employees, uni);
        got.sort_by(|a, b| a.1.cmp(&b.1));
        assert_eq!(
            got,
            vec![
                (None, "Alice".into()),
                (None, "Bob".into()),
                (Some(1), "Jonathan".into()),
                (Some(2), "Meir".into()),
                (Some(3), "Winston".into()),
            ]
        );
    }
}
