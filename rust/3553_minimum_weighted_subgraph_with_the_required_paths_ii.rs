/// LeetCode #3553 - Minimum Weighted Subgraph With the Required Paths II
use std::collections::VecDeque;

fn minimum_weight(edges: Vec<Vec<i32>>, queries: Vec<Vec<i32>>) -> Vec<i64> {
    let n = edges.len() + 1;
    let m = (n as f64).log2().ceil() as usize;
    let mut g = vec![Vec::new(); n];
    for e in &edges {
        let (u, v, w) = (e[0] as usize, e[1] as usize, e[2] as i64);
        g[u].push((v, w));
        g[v].push((u, w));
    }
    let mut jump = vec![vec![0usize; m.max(1)]; n];
    let mut depth = vec![0usize; n];
    let mut dist = vec![0i64; n];
    let mut q = VecDeque::from([0usize]);
    let mut parent = vec![usize::MAX; n];
    parent[0] = 0;
    while let Some(u) = q.pop_front() {
        jump[u][0] = if parent[u] == usize::MAX { 0 } else { parent[u] };
        for j in 1..m {
            jump[u][j] = jump[jump[u][j - 1]][j - 1];
        }
        for &(v, w) in &g[u] {
            if v == parent[u] {
                continue;
            }
            parent[v] = u;
            depth[v] = depth[u] + 1;
            dist[v] = dist[u] + w;
            q.push_back(v);
        }
    }
    let get_lca = |mut u: usize, mut v: usize| -> usize {
        if depth[u] > depth[v] {
            std::mem::swap(&mut u, &mut v);
        }
        let mut diff = depth[v] - depth[u];
        for j in 0..m {
            if (diff >> j) & 1 == 1 {
                v = jump[v][j];
            }
        }
        if u == v {
            return u;
        }
        for j in (0..m).rev() {
            if jump[u][j] != jump[v][j] {
                u = jump[u][j];
                v = jump[v][j];
            }
        }
        jump[u][0]
    };
    let distance = |u: usize, v: usize| -> i64 {
        let lca = get_lca(u, v);
        dist[u] + dist[v] - 2 * dist[lca]
    };
    queries
        .into_iter()
        .map(|q| {
            let (a, b, c) = (q[0] as usize, q[1] as usize, q[2] as usize);
            (distance(a, b) + distance(a, c) + distance(b, c)) / 2
        })
        .collect()
}

fn main() {
    println!(
        "{:?}",
        minimum_weight(
            vec![vec![0, 1, 2], vec![1, 2, 3], vec![1, 3, 5], vec![1, 4, 4], vec![2, 5, 6]],
            vec![vec![2, 3, 4], vec![0, 2, 5]]
        )
    );
}

#[cfg(test)]
mod tests {
    use super::minimum_weight;

    #[test]
    fn example1() {
        assert_eq!(
            minimum_weight(
                vec![vec![0, 1, 2], vec![1, 2, 3], vec![1, 3, 5], vec![1, 4, 4], vec![2, 5, 6]],
                vec![vec![2, 3, 4], vec![0, 2, 5]]
            ),
            vec![12, 11]
        );
    }

    #[test]
    fn example2() {
        assert_eq!(
            minimum_weight(vec![vec![1, 0, 8], vec![0, 2, 7]], vec![vec![0, 1, 2]]),
            vec![15]
        );
    }
}
