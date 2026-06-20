/// LeetCode #1986 - Minimum Number of Work Sessions to Finish the Tasks
const INF: i32 = i32::MAX / 2;

fn min_sessions(tasks: Vec<i32>, session_time: i32) -> i32 {
    let n = tasks.len();
    let mut ok = vec![false; 1 << n];
    for i in 1..(1 << n) {
        let t: i32 = (0..n)
            .filter(|&j| (i >> j) & 1 == 1)
            .map(|j| tasks[j])
            .sum();
        ok[i] = t <= session_time;
    }

    let mut f = vec![INF; 1 << n];
    f[0] = 0;
    for i in 1..(1 << n) {
        let mut j = i;
        while j > 0 {
            if ok[j] {
                f[i] = f[i].min(f[i ^ j] + 1);
            }
            j = (j - 1) & i;
        }
    }
    f[(1 << n) - 1]
}

fn main() {
    println!("{}", min_sessions(vec![1, 2, 3], 3));
}

#[cfg(test)]
mod tests {
    use super::min_sessions;

    #[test]
    fn example_one() {
        assert_eq!(min_sessions(vec![1, 2, 3], 3), 2);
    }

    #[test]
    fn example_two() {
        assert_eq!(min_sessions(vec![3, 1, 3, 1, 1], 8), 2);
    }
}
