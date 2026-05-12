/// LeetCode #690 - Employee Importance
use std::collections::HashMap;

pub struct Employee {
    pub id: i32,
    pub importance: i32,
    pub subordinates: Vec<i32>,
}

fn get_importance(employees: Vec<Employee>, id: i32) -> i32 {
    let map: HashMap<i32, &Employee> = employees.iter().map(|e| (e.id, e)).collect();
    fn dfs(id: i32, map: &HashMap<i32, &Employee>) -> i32 {
        let e = map[&id];
        let mut s = e.importance;
        for &sub in &e.subordinates {
            s += dfs(sub, map);
        }
        s
    }
    dfs(id, &map)
}

fn main() {
    let es = vec![
        Employee { id: 1, importance: 5, subordinates: vec![2, 3] },
        Employee { id: 2, importance: 3, subordinates: vec![] },
        Employee { id: 3, importance: 3, subordinates: vec![] },
    ];
    println!("{}", get_importance(es, 1));
}

#[cfg(test)]
mod tests {
    use super::{get_importance, Employee};

    #[test]
    fn example_one() {
        let es = vec![
            Employee { id: 1, importance: 5, subordinates: vec![2, 3] },
            Employee { id: 2, importance: 3, subordinates: vec![] },
            Employee { id: 3, importance: 3, subordinates: vec![] },
        ];
        assert_eq!(get_importance(es, 1), 11);
    }
}
