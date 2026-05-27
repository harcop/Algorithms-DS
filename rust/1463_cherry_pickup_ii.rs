/// LeetCode #1463 - Cherry Pickup Ii
fn cherry_pickup(grid: Vec<Vec<i32>>) -> i32 {
    let rows = grid.len();
    let cols = grid[0].len();
    let mut dp = vec![vec![0; cols]; cols];
    dp[0][cols - 1] = grid[0][0] + grid[0][cols - 1];
    for r in 1..rows {
        let mut ndp = vec![vec![i32::MIN / 4; cols]; cols];
        for c1 in 0..cols {
            for c2 in 0..cols {
                let mut best = i32::MIN / 4;
                for pc1 in c1.saturating_sub(1)..=(c1 + 1).min(cols - 1) {
                    for pc2 in c2.saturating_sub(1)..=(c2 + 1).min(cols - 1) {
                        if dp[pc1][pc2] > i32::MIN / 8 {
                            let mut cur = dp[pc1][pc2] + grid[r][c1];
                            if c2 != c1 { cur += grid[r][c2]; }
                            best = best.max(cur);
                        }
                    }
                }
                ndp[c1][c2] = best;
            }
        }
        dp = ndp;
    }
    *dp.iter().flat_map(|row| row.iter()).max().unwrap()
}
fn main() { println!("{}", cherry_pickup(vec![vec![3,1,1],vec![2,5,1],vec![1,5,5],vec![2,1,1]])); }
#[cfg(test)]
mod tests {
    use super::cherry_pickup;
    #[test]
    fn example_one() { assert_eq!(cherry_pickup(vec![vec![3,1,1],vec![2,5,1],vec![1,5,5],vec![2,1,1]]), 24); }
    #[test]
    fn example_two() { assert_eq!(cherry_pickup(vec![vec![1,0,0,0,0,0,1],vec![2,0,0,0,0,3,0],vec![2,0,9,0,0,0,0],vec![0,3,0,5,4,0,0],vec![1,0,2,3,0,0,6]]), 28); }
}