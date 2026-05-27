/// LeetCode #1473 - Paint House Iii
fn min_cost(_n: i32, _m: i32, cost: Vec<Vec<i32>>, k: i32) -> i32 {
    let houses = cost.len();
    let colors = cost[0].len();
    let k = k as usize;
    let inf = i32::MAX / 4;
    let mut dp = vec![vec![vec![inf; k + 1]; colors]; houses];
    for c in 0..colors { dp[0][c][1] = cost[0][c]; }
    for i in 1..houses {
        for c in 0..colors {
            for nb in 1..=k.min(i + 1) {
                for pc in 0..colors {
                    let prev = if c == pc { nb } else { nb - 1 };
                    if prev == 0 { continue; }
                    let extra = if c == pc { 0 } else { ((i + 1) * (c + 1)) as i32 };
                    dp[i][c][nb] = dp[i][c][nb].min(dp[i - 1][pc][prev] + cost[i][c] + extra);
                }
            }
        }
    }
    let mut ans = inf;
    for c in 0..colors {
        for nb in 1..=k {
            ans = ans.min(dp[houses - 1][c][nb]);
        }
    }
    ans
}
fn main() { println!("{}", min_cost(2, 3, vec![vec![1,5,3],vec![2,6,4]], 3)); }
#[cfg(test)]
mod tests {
    use super::min_cost;
    #[test]
    fn example_one() { assert_eq!(min_cost(2, 3, vec![vec![1,5,3],vec![2,6,4]], 3), 3); }
    #[test]
    fn example_two() { assert_eq!(min_cost(3, 1, vec![vec![1],vec![2],vec![3]], 3), 6); }
}