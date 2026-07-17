/// LeetCode #2432 - The Employee That Worked on the Longest Task
fn hardest_worker(_n: i32, logs: Vec<Vec<i32>>) -> i32 {
    let mut answer = logs[0][0];
    let mut longest = logs[0][1];

    for i in 1..logs.len() {
        let duration = logs[i][1] - logs[i - 1][1];
        let employee = logs[i][0];
        if duration > longest || (duration == longest && employee < answer) {
            longest = duration;
            answer = employee;
        }
    }

    answer
}

fn main() {
    println!(
        "{}",
        hardest_worker(10, vec![vec![0, 3], vec![2, 5], vec![0, 9], vec![1, 15]])
    );
}

#[cfg(test)]
mod tests {
    use super::hardest_worker;

    #[test]
    fn example_one() {
        assert_eq!(
            hardest_worker(10, vec![vec![0, 3], vec![2, 5], vec![0, 9], vec![1, 15]]),
            1
        );
    }

    #[test]
    fn breaks_tie_by_employee_id() {
        assert_eq!(
            hardest_worker(26, vec![vec![1, 1], vec![3, 7], vec![2, 12], vec![7, 17]]),
            3
        );
    }
}
