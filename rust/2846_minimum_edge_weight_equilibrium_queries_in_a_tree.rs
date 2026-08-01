use std::collections::VecDeque;

/// LeetCode #2846 - Minimum Edge Weight Equilibrium Queries in a Tree
fn min_operations_queries(
    n: i32,
    edges: Vec<Vec<i32>>,
    queries: Vec<Vec<i32>>,
) -> Vec<i32> {
    let n = n as usize;
    let levels = (usize::BITS - n.leading_zeros()) as usize;
    let mut graph = vec![Vec::new(); n];
    for edge in edges {
        let (u, v, weight) = (edge[0] as usize, edge[1] as usize, edge[2] as usize - 1);
        graph[u].push((v, weight));
        graph[v].push((u, weight));
    }

    let mut ancestors = vec![vec![0; levels]; n];
    let mut parent = vec![0; n];
    let mut depth = vec![0; n];
    let mut counts = vec![[0i32; 26]; n];
    let mut queue = VecDeque::from([0]);
    let mut visited = vec![false; n];
    visited[0] = true;

    while let Some(node) = queue.pop_front() {
        ancestors[node][0] = parent[node];
        for level in 1..levels {
            ancestors[node][level] =
                ancestors[ancestors[node][level - 1]][level - 1];
        }
        for &(next, weight) in &graph[node] {
            if visited[next] {
                continue;
            }
            visited[next] = true;
            parent[next] = node;
            depth[next] = depth[node] + 1;
            counts[next] = counts[node];
            counts[next][weight] += 1;
            queue.push_back(next);
        }
    }

    fn lca(
        mut u: usize,
        mut v: usize,
        depth: &[i32],
        ancestors: &[Vec<usize>],
    ) -> usize {
        if depth[u] < depth[v] {
            std::mem::swap(&mut u, &mut v);
        }
        let levels = ancestors[0].len();
        for level in (0..levels).rev() {
            if depth[u] - depth[v] >= (1i32 << level) {
                u = ancestors[u][level];
            }
        }
        if u == v {
            return u;
        }
        for level in (0..levels).rev() {
            if ancestors[u][level] != ancestors[v][level] {
                u = ancestors[u][level];
                v = ancestors[v][level];
            }
        }
        ancestors[u][0]
    }

    queries
        .into_iter()
        .map(|query| {
            let (u, v) = (query[0] as usize, query[1] as usize);
            let common = lca(u, v, &depth, &ancestors);
            let most_frequent = (0..26)
                .map(|weight| counts[u][weight] + counts[v][weight] - 2 * counts[common][weight])
                .max()
                .unwrap_or(0);
            depth[u] + depth[v] - 2 * depth[common] - most_frequent
        })
        .collect()
}

fn main() {
    let edges = vec![
        vec![0, 1, 1],
        vec![1, 2, 1],
        vec![2, 3, 1],
        vec![3, 4, 2],
        vec![4, 5, 2],
        vec![5, 6, 2],
    ];
    println!(
        "{:?}",
        min_operations_queries(7, edges, vec![vec![0, 3], vec![3, 6]])
    );
}

#[cfg(test)]
mod tests {
    use super::min_operations_queries;

    #[test]
    fn example_one() {
        let edges = vec![
            vec![0, 1, 1],
            vec![1, 2, 1],
            vec![2, 3, 1],
            vec![3, 4, 2],
            vec![4, 5, 2],
            vec![5, 6, 2],
        ];
        let queries = vec![vec![0, 3], vec![3, 6], vec![2, 6], vec![0, 6]];
        assert_eq!(min_operations_queries(7, edges, queries), vec![0, 0, 1, 3]);
    }

    #[test]
    fn example_two() {
        let edges = vec![
            vec![1, 2, 6],
            vec![1, 3, 4],
            vec![2, 4, 6],
            vec![2, 5, 3],
            vec![3, 6, 6],
            vec![3, 0, 8],
            vec![7, 0, 2],
        ];
        let queries = vec![vec![4, 6], vec![0, 4], vec![6, 5], vec![7, 4]];
        assert_eq!(min_operations_queries(8, edges, queries), vec![1, 2, 2, 3]);
    }
}
