/// LeetCode #1548 - The Most Similar Path In A Graph
fn most_similar(n: i32, roads: Vec<Vec<i32>>, names: Vec<String>, target: String) -> Vec<i32> {
    let n = n as usize;
    let mut g = vec![vec![]; n];
    for e in roads {
        let u = e[0] as usize;
        let v = e[1] as usize;
        g[u].push(v);
        g[v].push(u);
    }
    let m = target.len();
    let target = target.as_bytes();
    let mut dp = vec![vec![n; m]; n];
    let mut parent = vec![vec![0usize; m]; n];
    for i in 0..n {
        dp[i][0] = if names[i].as_bytes()[0] == target[0] { 0 } else { 1 };
    }
    for day in 1..m {
        let mut ndp = vec![vec![n; m]; n];
        for u in 0..n {
            for &v in &g[u] {
                let cost = dp[v][day - 1] + if names[u].as_bytes()[day] == target[day] { 0 } else { 1 };
                if cost < ndp[u][day] {
                    ndp[u][day] = cost;
                    parent[u][day] = v;
                }
            }
        }
        dp = ndp;
    }
    let mut u = (0..n).min_by_key(|&i| dp[i][m - 1]).unwrap();
    let mut path = vec![0i32; m];
    for day in (0..m).rev() {
        path[day] = u as i32;
        if day > 0 { u = parent[u][day]; }
    }
    path
}
fn main() {
    println!("{:?}", most_similar(5, vec![vec![0, 2], vec![0, 3], vec![1, 2], vec![1, 3], vec![1, 4], vec![2, 4]], vec!["ATL".into(), "PEK".into(), "LAX".into(), "ATL".into(), "DXB".into()], "ATL".into()));
}
#[cfg(test)]
mod tests {
    use super::most_similar;
    #[test]
    fn example_one() {
        assert_eq!(most_similar(5, vec![vec![0, 2], vec![0, 3], vec![1, 2], vec![1, 3], vec![1, 4], vec![2, 4]], vec!["ATL".into(), "PEK".into(), "LAX".into(), "ATL".into(), "DXB".into()], "ATL".into()), vec![0, 3, 0]);
    }
}
