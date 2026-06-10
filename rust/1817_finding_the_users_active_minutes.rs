/// LeetCode #1817 - Finding the Users Active Minutes
use std::collections::HashMap;
use std::collections::HashSet;

fn finding_users_active_minutes(logs: Vec<Vec<i32>>, k: i32) -> Vec<i32> {
    let mut users: HashMap<i32, HashSet<i32>> = HashMap::new();
    for log in logs {
        users.entry(log[0]).or_default().insert(log[1]);
    }
    let mut ans = vec![0i32; k as usize];
    for minutes in users.values() {
        ans[minutes.len() - 1] += 1;
    }
    ans
}

fn main() {
    println!(
        "{:?}",
        finding_users_active_minutes(vec![vec![0, 5], vec![1, 2], vec![0, 2], vec![0, 5], vec![1, 3]], 5)
    );
}

#[cfg(test)]
mod tests {
    use super::finding_users_active_minutes;

    #[test]
    fn example_one() {
        assert_eq!(
            finding_users_active_minutes(
                vec![vec![0, 5], vec![1, 2], vec![0, 2], vec![0, 5], vec![1, 3]],
                5,
            ),
            vec![0, 2, 0, 0, 0]
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(
            finding_users_active_minutes(vec![vec![1, 1], vec![2, 2], vec![2, 3]], 4),
            vec![1, 1, 0, 0]
        );
    }
}
