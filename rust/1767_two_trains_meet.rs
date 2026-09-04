/// LeetCode #1767 - Find the Subtasks That Did Not Execute (SQL; Rust analogue)
use std::collections::HashSet;

fn missing_subtasks(tasks: Vec<(i32, i32)>, executed: Vec<(i32, i32)>) -> Vec<(i32, i32)> {
    let done: HashSet<(i32, i32)> = executed.into_iter().collect();
    let mut ans = Vec::new();
    for (task_id, subtasks_count) in tasks {
        for subtask_id in 1..=subtasks_count {
            if !done.contains(&(task_id, subtask_id)) {
                ans.push((task_id, subtask_id));
            }
        }
    }
    ans.sort();
    ans
}

fn main() {
    println!(
        "{:?}",
        missing_subtasks(vec![(1, 3), (2, 2), (3, 4)], vec![(1, 2), (3, 1), (3, 2), (3, 3), (3, 4)])
    );
}

#[cfg(test)]
mod tests {
    use super::missing_subtasks;

    #[test]
    fn example_one() {
        let tasks = vec![(1, 3), (2, 2), (3, 4)];
        let executed = vec![(1, 2), (3, 1), (3, 2), (3, 3), (3, 4)];
        assert_eq!(
            missing_subtasks(tasks, executed),
            vec![(1, 1), (1, 3), (2, 1), (2, 2)]
        );
    }
}
