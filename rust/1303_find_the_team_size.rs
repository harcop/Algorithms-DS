/// LeetCode #1303 - Find the Team Size (SQL; Rust analogue)
use std::collections::HashMap;

fn team_size(employees: Vec<(i32, i32)>) -> Vec<(i32, i32)> {
    let mut cnt: HashMap<i32, i32> = HashMap::new();
    for &(_, team) in &employees {
        *cnt.entry(team).or_insert(0) += 1;
    }
    employees
        .into_iter()
        .map(|(id, team)| (id, cnt[&team]))
        .collect()
}

fn main() {
    println!("{:?}", team_size(vec![(1, 8), (2, 8), (3, 8), (4, 7), (5, 9), (6, 9)]));
}

#[cfg(test)]
mod tests {
    use super::team_size;

    #[test]
    fn example() {
        assert_eq!(
            team_size(vec![(1, 8), (2, 8), (3, 8), (4, 7), (5, 9), (6, 9)]),
            vec![(1, 3), (2, 3), (3, 3), (4, 1), (5, 2), (6, 2)]
        );
    }
}
