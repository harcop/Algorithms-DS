/// LeetCode #2895 - Minimum Processing Time
fn min_processing_time(mut processor_time: Vec<i32>, mut tasks: Vec<i32>) -> i32 {
    processor_time.sort_unstable();
    tasks.sort_unstable();
    let mut ans = 0;
    let mut i = tasks.len();
    for &t in &processor_time {
        i -= 4;
        ans = ans.max(t + tasks[i + 3]);
    }
    ans
}

fn main() {
    println!(
        "{}",
        min_processing_time(vec![8, 10], vec![2, 2, 3, 1, 8, 7, 4, 5])
    );
}

#[cfg(test)]
mod tests {
    use super::min_processing_time;

    #[test]
    fn example_one() {
        assert_eq!(
            min_processing_time(vec![8, 10], vec![2, 2, 3, 1, 8, 7, 4, 5]),
            16
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(
            min_processing_time(vec![10, 20], vec![2, 3, 1, 2, 5, 8, 4, 3]),
            23
        );
    }
}
