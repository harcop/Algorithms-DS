/// LeetCode #1376 - Time Needed To Inform All Employees

fn num_of_minutes(n: i32, head_id: i32, manager: Vec<i32>, inform_time: Vec<i32>) -> i32 {
    let n = n as usize;
    let head = head_id as usize;
    let mut children = vec![vec![]; n];
    for (emp, &mgr) in manager.iter().enumerate() {
        if mgr != -1 {
            children[mgr as usize].push(emp);
        }
    }
    fn dfs(u: usize, children: &[Vec<usize>], inform_time: &[i32]) -> i32 {
        let mut best = 0;
        for &v in &children[u] {
            best = best.max(dfs(v, children, inform_time));
        }
        inform_time[u] + best
    }
    dfs(head, &children, &inform_time)
}

fn main() {
    println!("{}", num_of_minutes(1, 0, vec![-1], vec![0]));
}

#[cfg(test)]
mod tests {
    use super::num_of_minutes;

    #[test]
    fn example_one() {
        assert_eq!(num_of_minutes(1, 0, vec![-1], vec![0]), 0);
    }

    #[test]
    fn example_two() {
        assert_eq!(
            num_of_minutes(6, 2, vec![2, 2, -1, 2, 2, 2], vec![0, 0, 1, 0, 0, 0]),
            1
        );
    }
}
