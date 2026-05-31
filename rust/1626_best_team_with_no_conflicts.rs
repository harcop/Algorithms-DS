/// LeetCode #1626 - Best Team With No Conflicts
fn best_team_score(scores: Vec<i32>, ages: Vec<i32>) -> i32 {
    let n = scores.len();
    let mut idx: Vec<usize> = (0..n).collect();
    idx.sort_unstable_by_key(|&i| (ages[i], scores[i]));
    let mut dp = vec![0i32; n];
    let mut ans = 0i32;
    for i in 0..n {
        dp[i] = scores[idx[i]];
        for j in 0..i {
            if ages[idx[j]] <= ages[idx[i]] && scores[idx[j]] <= scores[idx[i]] {
                dp[i] = dp[i].max(dp[j] + scores[idx[i]]);
            }
        }
        ans = ans.max(dp[i]);
    }
    ans
}
fn main() { println!("{}", best_team_score(vec![1,3,5,10,15], vec![1,2,3,4,5])); }
#[cfg(test)]
mod tests {
    use super::best_team_score;
    #[test]
    fn example_one() { assert_eq!(best_team_score(vec![1,3,5,10,15], vec![1,2,3,4,5]), 34); }
}