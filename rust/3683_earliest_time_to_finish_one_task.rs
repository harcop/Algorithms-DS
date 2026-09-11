/// LeetCode #3683 - Earliest Time to Finish One Task
fn earliest_time(tasks: Vec<Vec<i32>>) -> i32 {
    tasks.into_iter().map(|t| t[0] + t[1]).min().unwrap()
}

fn main() {
    println!("{}", earliest_time(vec![vec![1, 6], vec![2, 3]]));
}

#[cfg(test)]
mod tests {
    use super::earliest_time;

    #[test]
    fn example1() {
        assert_eq!(earliest_time(vec![vec![1, 6], vec![2, 3]]), 5);
    }

    #[test]
    fn example2() {
        assert_eq!(
            earliest_time(vec![vec![100, 100], vec![100, 100], vec![100, 100]]),
            200
        );
    }
}
