/// LeetCode #1478 - Allocate Mailboxes
fn min_distance(mut houses: Vec<i32>, k: i32) -> i32 {
    houses.sort_unstable();
    let n = houses.len();
    let k = k as usize;
    let mut cost = vec![vec![0i32; n]; n];
    for i in 0..n {
        for j in i..n {
            let mid = houses[(i + j) / 2];
            cost[i][j] = (i..=j).map(|x| (houses[x] - mid).abs()).sum();
        }
    }
    let inf = i32::MAX / 4;
    let mut dp = vec![vec![inf; k + 1]; n + 1];
    dp[0][0] = 0;
    for i in 1..=n {
        for boxes in 1..=k.min(i) {
            for j in boxes - 1..i {
                dp[i][boxes] = dp[i][boxes].min(dp[j][boxes - 1] + cost[j][i - 1]);
            }
        }
    }
    dp[n][k]
}
fn main() { println!("{}", min_distance(vec![1,4,8,10,20], 3)); }
#[cfg(test)]
mod tests {
    use super::min_distance;
    #[test]
    fn example_one() { assert_eq!(min_distance(vec![1,4,8,10,20], 3), 5); }
    #[test]
    fn example_two() { assert_eq!(min_distance(vec![2,3,5,12,18], 2), 9); }
}