/// LeetCode #1129 - Shortest Path with Alternating Colors
use std::collections::VecDeque;

fn shortest_alternating_paths(n: i32, red_edges: Vec<Vec<i32>>, blue_edges: Vec<Vec<i32>>) -> Vec<i32> {
    let n = n as usize;
    let mut red: Vec<Vec<usize>> = vec![Vec::new(); n];
    let mut blue: Vec<Vec<usize>> = vec![Vec::new(); n];
    for e in red_edges {
        red[e[0] as usize].push(e[1] as usize);
    }
    for e in blue_edges {
        blue[e[0] as usize].push(e[1] as usize);
    }
    let mut dist = vec![vec![-1i32; 2]; n];
    dist[0][0] = 0;
    dist[0][1] = 0;
    let mut q = VecDeque::new();
    q.push_back((0usize, 0usize));
    q.push_back((0usize, 1usize));
    while let Some((u, color)) = q.pop_front() {
        let d = dist[u][color];
        let next_color = 1 - color;
        let edges = if color == 0 { &red[u] } else { &blue[u] };
        for &v in edges {
            if dist[v][next_color] == -1 {
                dist[v][next_color] = d + 1;
                q.push_back((v, next_color));
            }
        }
    }
    (0..n)
        .map(|i| {
            let a = dist[i][0];
            let b = dist[i][1];
            match (a, b) {
                (-1, -1) => -1,
                (-1, _) => b,
                (_, -1) => a,
                _ => a.min(b),
            }
        })
        .collect()
}

fn main() {
    println!(
        "{:?}",
        shortest_alternating_paths(3, vec![vec![0, 1], vec![1, 2]], vec![vec![1, 0]])
    );
}

#[cfg(test)]
mod tests {
    use super::shortest_alternating_paths;

    #[test]
    fn example_one() {
        assert_eq!(
            shortest_alternating_paths(3, vec![vec![0, 1], vec![1, 2]], vec![vec![1, 0]]),
            vec![0, 1, 1]
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(
            shortest_alternating_paths(3, vec![vec![0, 1], vec![0, 2]], vec![vec![1, 0]]),
            vec![0, 1, -1]
        );
    }
}
