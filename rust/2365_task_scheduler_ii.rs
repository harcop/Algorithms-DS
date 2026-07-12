/// LeetCode #2365 - Task Scheduler II
use std::collections::HashMap;

fn task_scheduler_ii(tasks: Vec<i32>, space: i32) -> i64 {
    let mut next_available: HashMap<i32, i64> = HashMap::new();
    let mut ans = 0i64;

    for task in tasks {
        ans = (ans + 1).max(*next_available.get(&task).unwrap_or(&0));
        next_available.insert(task, ans + space as i64 + 1);
    }

    ans
}

fn main() {
    println!("{}", task_scheduler_ii(vec![1, 2, 1, 2, 3, 1], 3));
}

#[cfg(test)]
mod tests {
    use super::task_scheduler_ii;

    #[test]
    fn example_one() {
        assert_eq!(task_scheduler_ii(vec![1, 2, 1, 2, 3, 1], 3), 9);
    }

    #[test]
    fn example_two() {
        assert_eq!(task_scheduler_ii(vec![5, 8, 8, 5], 2), 6);
    }
}
