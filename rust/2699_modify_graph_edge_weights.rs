/// LeetCode #2699 - Modify Graph Edge Weights
const INF: i64 = 2_000_000_000;

fn dijkstra(edges: &[Vec<i32>], n: usize, source: usize, destination: usize) -> i64 {
    let mut g = vec![vec![INF; n]; n];
    for e in edges {
        let a = e[0] as usize;
        let b = e[1] as usize;
        let w = e[2] as i64;
        if w == -1 {
            continue;
        }
        g[a][b] = w;
        g[b][a] = w;
    }
    let mut dist = vec![INF; n];
    let mut vis = vec![false; n];
    dist[source] = 0;
    for _ in 0..n {
        let mut k = usize::MAX;
        for j in 0..n {
            if !vis[j] && (k == usize::MAX || dist[j] < dist[k]) {
                k = j;
            }
        }
        if k == usize::MAX {
            break;
        }
        vis[k] = true;
        for j in 0..n {
            dist[j] = dist[j].min(dist[k] + g[k][j]);
        }
    }
    dist[destination]
}

fn modified_graph_edges(
    n: i32,
    mut edges: Vec<Vec<i32>>,
    source: i32,
    destination: i32,
    target: i32,
) -> Vec<Vec<i32>> {
    let n = n as usize;
    let source = source as usize;
    let destination = destination as usize;
    let target = target as i64;
    let mut d = dijkstra(&edges, n, source, destination);
    if d < target {
        return vec![];
    }
    let mut ok = d == target;
    for i in 0..edges.len() {
        if edges[i][2] > 0 {
            continue;
        }
        if ok {
            edges[i][2] = INF as i32;
            continue;
        }
        edges[i][2] = 1;
        d = dijkstra(&edges, n, source, destination);
        if d <= target {
            ok = true;
            edges[i][2] += (target - d) as i32;
        }
    }
    if ok {
        edges
    } else {
        vec![]
    }
}

fn main() {
    println!(
        "{:?}",
        modified_graph_edges(
            5,
            vec![
                vec![4, 1, -1],
                vec![2, 0, -1],
                vec![0, 3, -1],
                vec![4, 3, -1]
            ],
            0,
            1,
            5
        )
    );
}

#[cfg(test)]
mod tests {
    use super::{dijkstra, modified_graph_edges};

    #[test]
    fn example_one() {
        let edges = modified_graph_edges(
            5,
            vec![
                vec![4, 1, -1],
                vec![2, 0, -1],
                vec![0, 3, -1],
                vec![4, 3, -1],
            ],
            0,
            1,
            5,
        );
        assert!(!edges.is_empty());
        assert!(edges.iter().all(|e| e[2] > 0));
        assert_eq!(dijkstra(&edges, 5, 0, 1), 5);
    }

    #[test]
    fn example_two() {
        assert!(modified_graph_edges(3, vec![vec![0, 1, -1], vec![0, 2, 5]], 0, 2, 6).is_empty());
    }

    #[test]
    fn example_three() {
        let edges = modified_graph_edges(
            4,
            vec![
                vec![1, 0, 4],
                vec![1, 2, 3],
                vec![2, 3, 5],
                vec![0, 3, -1],
            ],
            0,
            2,
            6,
        );
        assert!(!edges.is_empty());
        assert_eq!(dijkstra(&edges, 4, 0, 2), 6);
    }
}
