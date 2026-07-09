/// LeetCode #2323 - Find Minimum Time to Finish All Jobs II
fn minimum_time(mut jobs: Vec<i32>, mut workers: Vec<i32>) -> i32 {
    jobs.sort_unstable();
    workers.sort_unstable();
    jobs.iter()
        .zip(workers.iter())
        .map(|(a, b)| (a + b - 1) / b)
        .max()
        .unwrap()
}

fn main() {
    println!("{}", minimum_time(vec![5, 2, 4], vec![1, 7, 5]));
}

#[cfg(test)]
mod tests {
    use super::minimum_time;

    #[test]
    fn example_one() {
        assert_eq!(minimum_time(vec![5, 2, 4], vec![1, 7, 5]), 2);
    }

    #[test]
    fn example_two() {
        assert_eq!(minimum_time(vec![3, 18, 15, 9], vec![6, 5, 1, 3]), 3);
    }
}
