/// LeetCode #826 - Most Profit Assigning Work
fn max_profit_assignment(difficulty: Vec<i32>, profit: Vec<i32>, worker: Vec<i32>) -> i32 {
    let mut jobs: Vec<(i32, i32)> = difficulty.into_iter().zip(profit).collect();
    jobs.sort_unstable();
    let mut max_p = 0;
    let mut ptr = 0;
    let mut worker = worker;
    worker.sort_unstable();
    let mut ans = 0;
    for w in worker {
        while ptr < jobs.len() && jobs[ptr].0 <= w {
            max_p = max_p.max(jobs[ptr].1);
            ptr += 1;
        }
        ans += max_p;
    }
    ans
}

fn main() {
    println!(
        "{}",
        max_profit_assignment(vec![2, 4, 6, 8, 10], vec![10, 20, 30, 40, 50], vec![4, 5, 6, 7])
    );
}

#[cfg(test)]
mod tests {
    use super::max_profit_assignment;

    #[test]
    fn example_one() {
        assert_eq!(
            max_profit_assignment(vec![2, 4, 6, 8, 10], vec![10, 20, 30, 40, 50], vec![4, 5, 6, 7]),
            100
        );
    }
}
