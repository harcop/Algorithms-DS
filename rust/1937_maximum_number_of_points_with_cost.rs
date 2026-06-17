/// LeetCode #1937 - Maximum Number of Points with Cost
fn max_points(points: Vec<Vec<i32>>) -> i64 {
    let rows = points.len();
    let cols = points[0].len();
    let mut dp: Vec<i64> = points[0].iter().map(|&x| x as i64).collect();

    for i in 1..rows {
        let mut ndp = vec![0i64; cols];
        for j in 0..cols {
            let mut best = i64::MIN / 2;
            for k in 0..cols {
                best = best.max(dp[k] - (j as i64 - k as i64).abs());
            }
            ndp[j] = best + points[i][j] as i64;
        }
        dp = ndp;
    }

    *dp.iter().max().unwrap()
}

fn main() {
    println!(
        "{}",
        max_points(vec![vec![1, 2, 3], vec![1, 5, 1], vec![3, 1, 1]])
    );
}

#[cfg(test)]
mod tests {
    use super::max_points;

    #[test]
    fn example_one() {
        assert_eq!(
            max_points(vec![vec![1, 2, 3], vec![1, 5, 1], vec![3, 1, 1]]),
            9
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(max_points(vec![vec![1, 5], vec![2, 3], vec![4, 2]]), 11);
    }
}
