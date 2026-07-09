/// LeetCode #2316 - Count Unreachable Pairs of Nodes in an Undirected Graph
fn count_pairs(n: i32, edges: Vec<Vec<i32>>) -> i64 {
    let n = n as usize;
    let mut g = vec![vec![]; n];
    let mut vis = vec![false; n];
    for e in edges {
        let u = e[0] as usize;
        let v = e[1] as usize;
        g[u].push(v);
        g[v].push(u);
    }

    fn dfs(g: &[Vec<usize>], vis: &mut [bool], u: usize) -> i64 {
        if vis[u] {
            return 0;
        }
        vis[u] = true;
        let mut cnt = 1i64;
        for &v in &g[u] {
            cnt += dfs(g, vis, v);
        }
        cnt
    }

    let mut ans = 0i64;
    let mut seen = 0i64;
    for u in 0..n {
        let t = dfs(&g, &mut vis, u);
        ans += t * seen;
        seen += t;
    }
    ans
}

fn main() {
    println!(
        "{}",
        count_pairs(3, vec![vec![0, 1], vec![0, 2], vec![1, 2]])
    );
}

#[cfg(test)]
mod tests {
    use super::count_pairs;

    #[test]
    fn example_one() {
        assert_eq!(
            count_pairs(3, vec![vec![0, 1], vec![0, 2], vec![1, 2]]),
            0
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(
            count_pairs(
                7,
                vec![vec![0, 2], vec![0, 5], vec![2, 4], vec![1, 6], vec![5, 4]]
            ),
            14
        );
    }
}
