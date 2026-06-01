/// LeetCode #1691 - Maximum Height By Stacking Cuboids
fn max_height(cuboids: Vec<Vec<i32>>) -> i32 {
    let mut v: Vec<[i32; 3]> = cuboids.iter().map(|c| {
        let mut a = [c[0], c[1], c[2]];
        a.sort_unstable();
        a
    }).collect();
    v.sort_unstable();
    let n = v.len();
    let mut dp = vec![0i32; n];
    for i in 0..n {
        dp[i] = v[i][2];
        for j in 0..i {
            if v[j][0] <= v[i][0] && v[j][1] <= v[i][1] && v[j][2] <= v[i][2] {
                dp[i] = dp[i].max(dp[j] + v[i][2]);
            }
        }
    }
    *dp.iter().max().unwrap()
}
fn main() { println!("{}", max_height(vec![vec![2,1,2],vec![3,1,2]])); }
#[cfg(test)]
mod tests {
    use super::max_height;
    #[test]
    fn example_one() { assert_eq!(max_height(vec![vec![2,1,2],vec![3,1,2]]), 5); }
}