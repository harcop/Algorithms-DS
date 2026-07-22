/// LeetCode #2589 - Minimum Time to Complete All Tasks
fn find_minimum_time(mut tasks: Vec<Vec<i32>>) -> i32 {
    tasks.sort_unstable_by_key(|t| t[1]);
    let mut vis = vec![0i32; 2010];
    let mut ans = 0;
    for task in tasks {
        let start = task[0] as usize;
        let end = task[1] as usize;
        let mut duration = task[2];
        for i in start..=end {
            duration -= vis[i];
        }
        let mut i = end;
        while i >= start && duration > 0 {
            if vis[i] == 0 {
                duration -= 1;
                vis[i] = 1;
                ans += 1;
            }
            if i == 0 {
                break;
            }
            i -= 1;
        }
    }
    ans
}

fn main() {
    println!(
        "{}",
        find_minimum_time(vec![vec![2, 3, 1], vec![4, 5, 1], vec![1, 5, 2]])
    );
}

#[cfg(test)]
mod tests {
    use super::find_minimum_time;

    #[test]
    fn example_one() {
        assert_eq!(
            find_minimum_time(vec![vec![2, 3, 1], vec![4, 5, 1], vec![1, 5, 2]]),
            2
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(
            find_minimum_time(vec![vec![1, 3, 2], vec![2, 5, 3], vec![5, 6, 2]]),
            4
        );
    }
}
