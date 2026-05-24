/// LeetCode #1335 - Minimum Difficulty of a Job Schedule
fn min_difficulty(job_difficulty: Vec<i32>, d: i32) -> i32 {
    let n = job_difficulty.len();
    let d = d as usize;
    if n < d {
        return -1;
    }
    let mut dp = vec![vec![i32::MAX; d + 1]; n];
    let mut mx = 0;
    for i in 0..n {
        mx = mx.max(job_difficulty[i]);
        dp[i][1] = mx;
    }
    for days in 2..=d {
        for i in days - 1..n {
            let mut cur = 0;
            for j in (days - 1..=i).rev() {
                cur = cur.max(job_difficulty[j]);
                if dp[j - 1][days - 1] < i32::MAX {
                    dp[i][days] = dp[i][days].min(dp[j - 1][days - 1] + cur);
                }
            }
        }
    }
    dp[n - 1][d]
}

fn main() {
    println!("{}", min_difficulty(vec![6, 5, 4, 3, 2, 1], 2));
}

#[cfg(test)]
mod tests {
    use super::min_difficulty;

    #[test]
    fn example_one() {
        assert_eq!(min_difficulty(vec![6, 5, 4, 3, 2, 1], 2), 7);
    }

    #[test]
    fn example_two() {
        assert_eq!(min_difficulty(vec![9, 9, 9], 4), -1);
    }
}
