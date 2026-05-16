/// LeetCode #847 - Shortest Path Visiting All Nodes
fn shortest_path_length(graph: Vec<Vec<i32>>) -> i32 {
    let n = graph.len();
    if n <= 1 {
        return 0;
    }
    use std::collections::{HashSet, VecDeque};
    let mut q = VecDeque::new();
    let mut seen = HashSet::new();
    for i in 0..n {
        let mask = 1 << i;
        q.push_back((i, mask, 0));
        seen.insert((i, mask));
    }
    while let Some((u, mask, d)) = q.pop_front() {
        if mask == (1 << n) - 1 {
            return d;
        }
        for &v in &graph[u] {
            let v = v as usize;
            let nmask = mask | (1 << v);
            if seen.insert((v, nmask)) {
                q.push_back((v, nmask, d + 1));
            }
        }
    }
    0
}

fn main() {
    println!(
        "{}",
        shortest_path_length(vec![vec![1, 2, 3], vec![0], vec![0], vec![0]])
    );
}

#[cfg(test)]
mod tests {
    use super::shortest_path_length;

    #[test]
    fn example_one() {
        assert_eq!(
            shortest_path_length(vec![vec![1, 2, 3], vec![0], vec![0], vec![0]]),
            4
        );
    }
}
