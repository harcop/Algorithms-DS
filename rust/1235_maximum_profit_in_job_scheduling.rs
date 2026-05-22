/// LeetCode #1235 - Maximum Profit in Job Scheduling
fn job_scheduling(start_time: Vec<i32>, end_time: Vec<i32>, profit: Vec<i32>) -> i32 {
    let n = start_time.len();
    let mut jobs: Vec<(i32, i32, i32)> = (0..n)
        .map(|i| (start_time[i], end_time[i], profit[i]))
        .collect();
    jobs.sort_by_key(|&(_, e, _)| e);
    let mut dp = vec![0i32; n];
    for i in 0..n {
        let (s, _, p) = jobs[i];
        let mut take = p;
        let mut lo = 0usize;
        let mut hi = i;
        while lo < hi {
            let mid = (lo + hi) / 2;
            if jobs[mid].1 <= s {
                lo = mid + 1;
            } else {
                hi = mid;
            }
        }
        if lo > 0 {
            take = take.max(p + dp[lo - 1]);
        }
        if i > 0 {
            take = take.max(dp[i - 1]);
        }
        dp[i] = take;
    }
    dp[n - 1]
}

fn main() {
    println!(
        "{}",
        job_scheduling(vec![1, 2, 3, 3], vec![3, 4, 5, 6], vec![50, 10, 40, 70])
    );
}

#[cfg(test)]
mod tests {
    use super::job_scheduling;

    #[test]
    fn example_one() {
        assert_eq!(
            job_scheduling(vec![1, 2, 3, 3], vec![3, 4, 5, 6], vec![50, 10, 40, 70]),
            120
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(
            job_scheduling(vec![1, 2, 3, 4, 6], vec![3, 5, 10, 6, 9], vec![20, 20, 100, 70, 60]),
            150
        );
    }

    #[test]
    fn example_three() {
        assert_eq!(job_scheduling(vec![1, 1, 1], vec![2, 3, 4], vec![5, 6, 4]), 6);
    }
}
