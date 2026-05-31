/// LeetCode #1594 - Maximum Non Negative Product In A Matrix
fn max_product_path(grid: Vec<Vec<i32>>) -> i32 {
    const MOD: i64 = 1_000_000_007;
    let n = grid.len();
    let m = grid[0].len();
    let mut dp = vec![vec![(0i64, 0i64, 0i32); m]; n];
    dp[0][0] = (grid[0][0].max(0) as i64, grid[0][0].min(0) as i64, if grid[0][0] < 0 { 1 } else { 0 });
    for i in 0..n {
        for j in 0..m {
            if i == 0 && j == 0 { continue; }
            let mut mx = i64::MIN / 4;
            let mut mn = i64::MAX / 4;
            let mut z = i32::MAX;
            if i > 0 {
                mx = mx.max(dp[i-1][j].0).max(dp[i-1][j].1);
                mn = mn.min(dp[i-1][j].0).min(dp[i-1][j].1);
                z = z.min(dp[i-1][j].2);
            }
            if j > 0 {
                mx = mx.max(dp[i][j-1].0).max(dp[i][j-1].1);
                mn = mn.min(dp[i][j-1].0).min(dp[i][j-1].1);
                z = z.min(dp[i][j-1].2);
            }
            let v = grid[i][j] as i64;
            if v >= 0 {
                dp[i][j] = (mx * v, mn * v, z);
            } else {
                dp[i][j] = (mn * v, mx * v, z + if v < 0 { 1 } else { 0 });
            }
        }
    }
    let (mx, _, z) = dp[n-1][m-1];
    if z % 2 == 1 && mx <= 0 { -1 } else { (mx % MOD) as i32 }
}
fn main() { println!("{}", max_product_path(vec![vec![-1,-2,-3],vec![-2,-3,-3],vec![-3,-3,-2]])); }
#[cfg(test)]
mod tests {
    use super::max_product_path;
    #[test]
    fn example_one() { assert_eq!(max_product_path(vec![vec![-1,-2,-3],vec![-2,-3,-3],vec![-3,-3,-2]]), -1); }
}