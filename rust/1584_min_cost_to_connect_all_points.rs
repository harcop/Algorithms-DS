/// LeetCode #1584 - Min Cost To Connect All Points
fn min_cost_connect_points(points: Vec<Vec<i32>>) -> i32 {
    let n = points.len();
    let dist = |i: usize, j: usize| {
        (points[i][0] - points[j][0]).abs() + (points[i][1] - points[j][1]).abs()
    };
    let mut used = vec![false; n];
    let mut min_d = vec![i32::MAX; n];
    min_d[0] = 0;
    let mut ans = 0i32;
    for _ in 0..n {
        let u = (0..n).filter(|&i| !used[i]).min_by_key(|&i| min_d[i]).unwrap();
        used[u] = true;
        ans += min_d[u];
        for v in 0..n {
            if !used[v] { min_d[v] = min_d[v].min(dist(u, v)); }
        }
    }
    ans
}
fn main() { println!("{}", min_cost_connect_points(vec![vec![0,0],vec![2,2],vec![3,10],vec![5,2],vec![7,0]])); }
#[cfg(test)]
mod tests {
    use super::min_cost_connect_points;
    #[test]
    fn example_one() { assert_eq!(min_cost_connect_points(vec![vec![0,0],vec![2,2],vec![3,10],vec![5,2],vec![7,0]]), 20); }
}