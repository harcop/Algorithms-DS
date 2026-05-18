/// LeetCode #882 - Reachable Nodes In Subdivided Graph
use std::cmp::Ordering;
use std::collections::BinaryHeap;

#[derive(Eq, PartialEq)]
struct State {
    dist: i32,
    node: usize,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.dist.cmp(&self.dist)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn dijkstra(n: usize, edges: &[Vec<i32>]) -> Vec<i32> {
    let mut adj: Vec<Vec<(usize, i32)>> = vec![vec![]; n];
    for e in edges {
        let u = e[0] as usize;
        let v = e[1] as usize;
        let w = e[2] + 1;
        adj[u].push((v, w));
        adj[v].push((u, w));
    }
    let mut dist = vec![i32::MAX; n];
    dist[0] = 0;
    let mut heap = BinaryHeap::new();
    heap.push(State { dist: 0, node: 0 });

    while let Some(State { dist: d, node }) = heap.pop() {
        if d != dist[node] {
            continue;
        }
        for &(to, w) in &adj[node] {
            let nd = d + w;
            if nd < dist[to] {
                dist[to] = nd;
                heap.push(State { dist: nd, node: to });
            }
        }
    }
    dist
}

fn reachable_nodes(n: i32, edges: Vec<Vec<i32>>, max_moves: i32) -> i32 {
    let n = n as usize;
    let max_moves = max_moves;
    let dist = dijkstra(n, &edges);
    let mut ans = 0i32;

    for i in 0..n {
        if dist[i] <= max_moves {
            ans += 1;
        }
    }

    for e in &edges {
        let u = e[0] as usize;
        let v = e[1] as usize;
        let cnt = e[2];
        let du = if dist[u] == i32::MAX { -1 } else { dist[u] };
        let dv = if dist[v] == i32::MAX { -1 } else { dist[v] };

        let a = if du < 0 { 0 } else { (max_moves - du).max(0).min(cnt) };
        let b = if dv < 0 { 0 } else { (max_moves - dv).max(0).min(cnt) };

        ans += a + b - (a + b).saturating_sub(cnt);
    }
    ans
}

fn main() {
    println!(
        "{}",
        reachable_nodes(7, vec![vec![0, 1, 2], vec![0, 2, 5], vec![2, 3, 1], vec![1, 3, 1]], 6)
    );
}

#[cfg(test)]
mod tests {
    use super::reachable_nodes;

    #[test]
    fn example_one() {
        assert_eq!(
            reachable_nodes(
                7,
                vec![vec![0, 1, 2], vec![0, 2, 5], vec![2, 3, 1], vec![1, 3, 1]],
                6
            ),
            18
        );
    }
}
