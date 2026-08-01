/// LeetCode #2872 - Maximum Number of K-Divisible Components
fn max_k_divisible_components(
    n: i32,
    edges: Vec<Vec<i32>>,
    values: Vec<i32>,
    k: i32,
) -> i32 {
    let n = n as usize;
    let mut graph = vec![Vec::new(); n];
    for edge in edges {
        let a = edge[0] as usize;
        let b = edge[1] as usize;
        graph[a].push(b);
        graph[b].push(a);
    }

    let mut parent = vec![usize::MAX; n];
    let mut order = Vec::with_capacity(n);
    parent[0] = 0;
    let mut stack = vec![0];
    while let Some(node) = stack.pop() {
        order.push(node);
        for &neighbor in &graph[node] {
            if parent[neighbor] == usize::MAX {
                parent[neighbor] = node;
                stack.push(neighbor);
            }
        }
    }

    let mut subtree_sum: Vec<i64> = values.into_iter().map(i64::from).collect();
    let mut components = 0;
    for &node in order.iter().rev() {
        if subtree_sum[node] % k as i64 == 0 {
            components += 1;
        } else if node != 0 {
            subtree_sum[parent[node]] += subtree_sum[node];
        }
    }
    components
}

fn main() {
    println!(
        "{}",
        max_k_divisible_components(
            5,
            vec![vec![0, 2], vec![1, 2], vec![1, 3], vec![2, 4]],
            vec![1, 8, 1, 4, 4],
            6
        )
    );
}

#[cfg(test)]
mod tests {
    use super::max_k_divisible_components;

    #[test]
    fn example_one() {
        assert_eq!(
            max_k_divisible_components(
                5,
                vec![vec![0, 2], vec![1, 2], vec![1, 3], vec![2, 4]],
                vec![1, 8, 1, 4, 4],
                6
            ),
            2
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(
            max_k_divisible_components(
                7,
                vec![
                    vec![0, 1],
                    vec![0, 2],
                    vec![1, 3],
                    vec![1, 4],
                    vec![2, 5],
                    vec![2, 6]
                ],
                vec![3, 0, 6, 1, 5, 2, 1],
                3
            ),
            3
        );
    }
}
