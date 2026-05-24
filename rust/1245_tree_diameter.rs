/// LeetCode #1245 - Tree Diameter
use std::collections::{HashMap, VecDeque};

fn tree_diameter(edges: Vec<Vec<i32>>) -> i32 {
    if edges.is_empty() {
        return 0;
    }
    let n = edges.len() + 1;
    let mut adj: Vec<Vec<usize>> = vec![vec![]; n];
    for e in &edges {
        let u = e[0] as usize;
        let v = e[1] as usize;
        adj[u].push(v);
        adj[v].push(u);
    }
    fn bfs(start: usize, adj: &[Vec<usize>]) -> (usize, i32) {
        let mut dist = vec![-1i32; adj.len()];
        let mut q = VecDeque::new();
        dist[start] = 0;
        q.push_back(start);
        let mut far = start;
        while let Some(u) = q.pop_front() {
            far = u;
            for &v in &adj[u] {
                if dist[v] == -1 {
                    dist[v] = dist[u] + 1;
                    q.push_back(v);
                }
            }
        }
        (far, dist[far])
    }
    let (a, _) = bfs(0, &adj);
    let (_, d) = bfs(a, &adj);
    d
}

fn main() {
    println!("{}", tree_diameter(vec![vec![0, 1], vec![1, 2]]));
}

#[cfg(test)]
mod tests {
    use super::tree_diameter;

    #[test]
    fn example_one() {
        assert_eq!(tree_diameter(vec![vec![0, 1], vec![1, 2]]), 2);
    }

    #[test]
    fn example_two() {
        assert_eq!(tree_diameter(vec![vec![0, 1], vec![0, 2]]), 2);
    }
}
