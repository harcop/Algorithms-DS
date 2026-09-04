/// LeetCode #1076 - Project Employees II (SQL; Rust analogue)
use std::collections::HashMap;

fn project_employees_ii(project: Vec<(i32, i32)>) -> Vec<i32> {
    let mut cnt: HashMap<i32, i32> = HashMap::new();
    for (pid, _) in project {
        *cnt.entry(pid).or_insert(0) += 1;
    }
    let mx = cnt.values().copied().max().unwrap_or(0);
    let mut ans: Vec<i32> = cnt.into_iter().filter(|(_, c)| *c == mx).map(|(p, _)| p).collect();
    ans.sort();
    ans
}

fn main() {
    println!("ok");
}

#[cfg(test)]
mod tests {
    use super::project_employees_ii;

    #[test]
    fn example() {
        let project = vec![(1, 1), (1, 2), (1, 3), (2, 1), (2, 4)];
        assert_eq!(project_employees_ii(project), vec![1]);
    }
}
