/// LeetCode #2065 - Maximum Path Quality of a Graph
fn maximal_path_quality(values: Vec<i32>, edges: Vec<Vec<i32>>, max_time: i32) -> i32 {
    let n = values.len();
    let mut g = vec![Vec::new(); n];
    for e in edges {
        let u = e[0] as usize;
        let v = e[1] as usize;
        let t = e[2];
        g[u].push((v, t));
        g[v].push((u, t));
    }

    let mut vis = vec![false; n];
    vis[0] = true;
    let mut ans = 0i32;

    fn dfs(
        u: usize,
        cost: i32,
        value: i32,
        values: &[i32],
        g: &[Vec<(usize, i32)>],
        vis: &mut [bool],
        max_time: i32,
        ans: &mut i32,
    ) {
        if u == 0 {
            *ans = (*ans).max(value);
        }
        for &(v, t) in &g[u] {
            if cost + t <= max_time {
                if vis[v] {
                    dfs(v, cost + t, value, values, g, vis, max_time, ans);
                } else {
                    vis[v] = true;
                    dfs(v, cost + t, value + values[v], values, g, vis, max_time, ans);
                    vis[v] = false;
                }
            }
        }
    }

    dfs(0, 0, values[0], &values, &g, &mut vis, max_time, &mut ans);
    ans
}

fn main() {
    println!(
        "{}",
        maximal_path_quality(
            vec![0, 32, 10, 43],
            vec![vec![0, 1, 10], vec![1, 2, 15], vec![0, 3, 10]],
            49,
        )
    );
}

#[cfg(test)]
mod tests {
    use super::maximal_path_quality;

    #[test]
    fn example_one() {
        assert_eq!(
            maximal_path_quality(
                vec![0, 32, 10, 43],
                vec![vec![0, 1, 10], vec![1, 2, 15], vec![0, 3, 10]],
                49,
            ),
            75
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(
            maximal_path_quality(
                vec![5, 10, 15, 20],
                vec![vec![0, 1, 10], vec![1, 2, 10], vec![0, 3, 10]],
                30,
            ),
            25
        );
    }
}
