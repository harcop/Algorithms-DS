/// LeetCode #1723 - Find Minimum Time to Finish All Jobs
use std::collections::HashSet;

fn minimum_time_required(mut jobs: Vec<i32>, k: i32) -> i32 {
    jobs.sort_unstable_by(|a, b| b.cmp(a));
    let k = k as usize;
    let mut workers = vec![0i32; k];
    let mut best = i32::MAX;
    fn dfs(i: usize, jobs: &[i32], workers: &mut [i32], best: &mut i32) {
        if i == jobs.len() {
            *best = (*best).min(*workers.iter().max().unwrap());
            return;
        }
        let mut used = HashSet::new();
        for j in 0..workers.len() {
            if !used.insert(workers[j]) {
                continue;
            }
            workers[j] += jobs[i];
            if workers[j] < *best {
                dfs(i + 1, jobs, workers, best);
            }
            workers[j] -= jobs[i];
        }
    }
    dfs(0, &jobs, &mut workers, &mut best);
    best
}
fn main() {
    println!("{}", minimum_time_required(vec![3, 2, 3], 3));
}
#[cfg(test)]
mod tests {
    use super::minimum_time_required;
    #[test]
    fn example_one() {
        assert_eq!(minimum_time_required(vec![3, 2, 3], 3), 3);
    }
    #[test]
    fn example_two() {
        assert_eq!(minimum_time_required(vec![1, 2, 4, 7, 8], 2), 11);
    }
}
