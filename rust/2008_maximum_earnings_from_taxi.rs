/// LeetCode #2008 - Maximum Earnings From Taxi
fn max_taxi_earnings(_n: i32, rides: Vec<Vec<i32>>) -> i64 {
    let mut rides = rides;
    rides.sort_by_key(|r| r[0]);
    let m = rides.len();
    let mut memo = vec![-1i64; m];

    fn dfs(i: usize, rides: &[Vec<i32>], memo: &mut [i64]) -> i64 {
        if i >= rides.len() {
            return 0;
        }
        if memo[i] != -1 {
            return memo[i];
        }
        let st = rides[i][0] as i64;
        let ed = rides[i][1] as i64;
        let tip = rides[i][2] as i64;
        let j = rides[i + 1..]
            .partition_point(|r| r[0] < rides[i][1])
            + i
            + 1;
        let ans = dfs(i + 1, rides, memo).max(dfs(j, rides, memo) + ed - st + tip);
        memo[i] = ans;
        ans
    }

    dfs(0, &rides, &mut memo)
}

fn main() {
    println!("{}", max_taxi_earnings(5, vec![vec![2, 5, 4], vec![1, 5, 1]]));
}

#[cfg(test)]
mod tests {
    use super::max_taxi_earnings;

    #[test]
    fn example_one() {
        assert_eq!(max_taxi_earnings(5, vec![vec![2, 5, 4], vec![1, 5, 1]]), 7);
    }

    #[test]
    fn example_two() {
        assert_eq!(
            max_taxi_earnings(
                20,
                vec![
                    vec![1, 6, 1],
                    vec![3, 10, 2],
                    vec![10, 12, 3],
                    vec![11, 12, 2],
                    vec![12, 15, 2],
                    vec![13, 18, 1],
                ],
            ),
            20
        );
    }
}
