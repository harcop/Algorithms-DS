/// LeetCode #3538 - Merge Operations for Minimum Travel Time
fn min_travel_time(_l: i32, n: i32, k: i32, position: Vec<i32>, time: Vec<i32>) -> i32 {
    let n = n as usize;
    let k = k as usize;
    let mut prefix = vec![0i64; n];
    prefix[0] = time[0] as i64;
    for i in 1..n {
        prefix[i] = prefix[i - 1] + time[i] as i64;
    }
    const INF: i64 = 1_000_000_000_000;
    let mut memo = vec![vec![vec![-1i64; n]; k + 1]; n];
    fn dp(
        i: usize,
        skips: usize,
        last: usize,
        n: usize,
        position: &[i32],
        prefix: &[i64],
        memo: &mut [Vec<Vec<i64>>],
    ) -> i64 {
        if memo[i][skips][last] != -1 {
            return memo[i][skips][last];
        }
        if i == n - 1 {
            let v = if skips == 0 { 0 } else { INF };
            memo[i][skips][last] = v;
            return v;
        }
        let mut res = INF;
        let rate = prefix[i] - if last > 0 { prefix[last - 1] } else { 0 };
        let end = (n - 1).min(i + skips + 1);
        for j in i + 1..=end {
            let distance = (position[j] - position[i]) as i64;
            let used = j - i - 1;
            res = res.min(distance * rate + dp(j, skips - used, i + 1, n, position, prefix, memo));
        }
        memo[i][skips][last] = res;
        res
    }
    dp(0, k, 0, n, &position, &prefix, &mut memo) as i32
}

fn main() {
    println!(
        "{}",
        min_travel_time(10, 4, 1, vec![0, 3, 8, 10], vec![5, 8, 3, 6])
    );
}

#[cfg(test)]
mod tests {
    use super::min_travel_time;

    #[test]
    fn example1() {
        assert_eq!(
            min_travel_time(10, 4, 1, vec![0, 3, 8, 10], vec![5, 8, 3, 6]),
            62
        );
    }

    #[test]
    fn example2() {
        assert_eq!(
            min_travel_time(5, 5, 1, vec![0, 1, 2, 3, 5], vec![8, 3, 9, 3, 3]),
            34
        );
    }
}
