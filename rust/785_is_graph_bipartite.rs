/// LeetCode #785 - Is Graph Bipartite?
fn is_bipartite(graph: Vec<Vec<i32>>) -> bool {
    let n = graph.len();
    let mut color = vec![-1i32; n];
    for start in 0..n {
        if color[start] != -1 {
            continue;
        }
        color[start] = 0;
        let mut stack = vec![start];
        while let Some(u) = stack.pop() {
            for &v in &graph[u] {
                let v = v as usize;
                if color[v] == -1 {
                    color[v] = 1 - color[u];
                    stack.push(v);
                } else if color[v] == color[u] {
                    return false;
                }
            }
        }
    }
    true
}

fn main() {
    println!("{}", is_bipartite(vec![vec![1, 3], vec![0, 2], vec![1, 3], vec![0, 2]]));
}

#[cfg(test)]
mod tests {
    use super::is_bipartite;

    #[test]
    fn example_one() {
        assert!(is_bipartite(vec![vec![1, 3], vec![0, 2], vec![1, 3], vec![0, 2]]));
    }

    #[test]
    fn example_two() {
        assert!(!is_bipartite(vec![vec![1, 2, 3], vec![0, 2], vec![0, 1, 3], vec![0, 2]]));
    }
}
