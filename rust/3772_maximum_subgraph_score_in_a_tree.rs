/// LeetCode #3772 - Maximum Subgraph Score in a Tree
fn max_subgraph_score(n: i32, edges: Vec<Vec<i32>>, good: Vec<i32>) -> Vec<i32> {
    let n = n as usize;
    let mut adj = vec![Vec::new(); n];
    for e in edges {
        let u = e[0] as usize;
        let v = e[1] as usize;
        adj[u].push(v);
        adj[v].push(u);
    }
    let mut parent = vec![usize::MAX; n];
    let mut q = vec![0];
    parent[0] = 0;
    let mut i = 0;
    while i < q.len() {
        let u = q[i];
        for &v in &adj[u] {
            if v == parent[u] {
                continue;
            }
            parent[v] = u;
            q.push(v);
        }
        i += 1;
    }
    parent[0] = usize::MAX;
    let mut dp: Vec<i32> = good.iter().map(|&x| if x == 1 { 1 } else { -1 }).collect();
    for &u in q.iter().rev() {
        if parent[u] == usize::MAX {
            continue;
        }
        dp[parent[u]] += dp[u].max(0);
    }
    for &u in &q {
        if parent[u] == usize::MAX {
            continue;
        }
        dp[u] += (dp[parent[u]] - dp[u].max(0)).max(0);
    }
    dp
}

fn main() {
    println!(
        "{:?}",
        max_subgraph_score(3, vec![vec![0, 1], vec![1, 2]], vec![1, 0, 1])
    );
}

#[cfg(test)]
mod tests {
    use super::max_subgraph_score;

    #[test]
    fn example1() {
        assert_eq!(
            max_subgraph_score(3, vec![vec![0, 1], vec![1, 2]], vec![1, 0, 1]),
            vec![1, 1, 1]
        );
    }

    #[test]
    fn example2() {
        assert_eq!(
            max_subgraph_score(
                5,
                vec![vec![1, 0], vec![1, 2], vec![1, 3], vec![3, 4]],
                vec![0, 1, 0, 1, 1]
            ),
            vec![2, 3, 2, 3, 3]
        );
    }

    #[test]
    fn example3() {
        assert_eq!(
            max_subgraph_score(2, vec![vec![0, 1]], vec![0, 0]),
            vec![-1, -1]
        );
    }
}
