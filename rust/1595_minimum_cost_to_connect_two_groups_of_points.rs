/// LeetCode #1595 - Minimum Cost To Connect Two Groups Of Points
fn connect_two_groups(cost: Vec<Vec<i32>>) -> i32 {
    let n = cost.len();
    let m = cost[0].len();
    let full = (1usize << m) - 1;
    let mut dp = vec![i32::MAX / 4; 1 << m];
    dp[0] = 0;
    for i in 0..n {
        let mut ndp = vec![i32::MAX / 4; 1 << m];
        for mask in 0usize..(1 << m) {
            if dp[mask] >= i32::MAX / 8 { continue; }
            for j in 0..m {
                let nmask = mask | (1 << j);
                ndp[nmask] = ndp[nmask].min(dp[mask] + cost[i][j]);
            }
        }
        dp = ndp;
    }
    dp[full]
}
fn main() { println!("{}", connect_two_groups(vec![vec![15,96],vec![36,2]])); }
#[cfg(test)]
mod tests {
    use super::connect_two_groups;
    #[test]
    fn example_one() { assert_eq!(connect_two_groups(vec![vec![15,96],vec![36,2]]), 17); }
    #[test]
    fn example_two() { assert_eq!(connect_two_groups(vec![vec![1,3,5],vec![4,1,1],vec![4,4,1],vec![2,5,1]]), 4); }
}