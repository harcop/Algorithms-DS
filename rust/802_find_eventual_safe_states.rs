/// LeetCode #802 - Find Eventual Safe States
fn eventual_safe_nodes(graph: Vec<Vec<i32>>) -> Vec<i32> {
    let n = graph.len();
    let mut state = vec![0u8; n];
    let mut safe = vec![false; n];
    fn dfs(u: usize, graph: &Vec<Vec<i32>>, state: &mut Vec<u8>, safe: &mut Vec<bool>) -> bool {
        match state[u] {
            1 => return false,
            2 => return safe[u],
            _ => {}
        }
        state[u] = 1;
        for &v in &graph[u] {
            if !dfs(v as usize, graph, state, safe) {
                state[u] = 2;
                safe[u] = false;
                return false;
            }
        }
        state[u] = 2;
        safe[u] = true;
        true
    }
    for i in 0..n {
        dfs(i, &graph, &mut state, &mut safe);
    }
    (0..n as i32).filter(|&i| safe[i as usize]).collect()
}

fn main() {
    let g = vec![vec![1, 2], vec![2, 3], vec![5], vec![0], vec![5], vec![]];
    println!("{:?}", eventual_safe_nodes(g));
}

#[cfg(test)]
mod tests {
    use super::eventual_safe_nodes;

    #[test]
    fn example_one() {
        let g = vec![vec![1, 2], vec![2, 3], vec![5], vec![0], vec![5], vec![]];
        assert_eq!(eventual_safe_nodes(g), vec![2, 4, 5]);
    }
}
