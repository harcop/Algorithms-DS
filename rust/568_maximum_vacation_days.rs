/// LeetCode #568 - Maximum Vacation Days
fn max_vacation_days(flights: Vec<Vec<i32>>, days: Vec<Vec<i32>>) -> i32 {
    let k = days.len();
    let n = days[0].len();
    let mut can = vec![vec![false; k]; k];
    for i in 0..k {
        can[i][i] = true;
        for j in 0..k {
            if flights[i][j] == 1 {
                can[i][j] = true;
            }
        }
    }
    let mut dp = vec![vec![-1i32; k]; n + 1];
    dp[n] = vec![0; k];
    for week in (0..n).rev() {
        for city in 0..k {
            let mut best = -1;
            for dest in 0..k {
                if can[city][dest] && dp[week + 1][dest] >= 0 {
                    best = best.max(days[dest][week] + dp[week + 1][dest]);
                }
            }
            dp[week][city] = best;
        }
    }
    dp[0][0].max(0)
}

fn main() {
    let flights = vec![vec![0, 1, 1], vec![1, 0, 1], vec![1, 1, 0]];
    let days = vec![vec![1, 3, 1], vec![6, 0, 3], vec![3, 3, 3]];
    println!("{}", max_vacation_days(flights, days));
}

#[cfg(test)]
mod tests {
    use super::max_vacation_days;

    #[test]
    fn example_one() {
        let flights = vec![vec![0, 1, 1], vec![1, 0, 1], vec![1, 1, 0]];
        let days = vec![vec![1, 3, 1], vec![6, 0, 3], vec![3, 3, 3]];
        assert_eq!(max_vacation_days(flights, days), 12);
    }

    #[test]
    fn example_two() {
        let flights = vec![vec![0, 0, 0], vec![0, 0, 0], vec![0, 0, 0]];
        let days = vec![vec![1, 1, 1], vec![7, 7, 7], vec![7, 7, 7]];
        assert_eq!(max_vacation_days(flights, days), 3);
    }

    #[test]
    fn example_three() {
        let flights = vec![vec![0, 1, 1], vec![1, 0, 1], vec![1, 1, 0]];
        let days = vec![vec![7, 0, 0], vec![0, 7, 0], vec![0, 0, 7]];
        assert_eq!(max_vacation_days(flights, days), 21);
    }
}
