/// LeetCode #2858 - Minimum Edge Reversals So Every Node Is Reachable
fn min_edge_reversals(n: i32, edges: Vec<Vec<i32>>) -> Vec<i32> {
    let mut graph = vec![Vec::new(); n as usize];
    for edge in edges {
        let (from, to) = (edge[0] as usize, edge[1] as usize);
        graph[from].push((to, 1));
        graph[to].push((from, -1));
    }

    fn initial_cost(node: usize, parent: usize, graph: &[Vec<(usize, i32)>]) -> i32 {
        graph[node]
            .iter()
            .filter(|&&(neighbor, _)| neighbor != parent)
            .map(|&(neighbor, direction)| {
                i32::from(direction == -1) + initial_cost(neighbor, node, graph)
            })
            .sum()
    }

    fn reroot(
        node: usize,
        parent: usize,
        graph: &[Vec<(usize, i32)>],
        answer: &mut [i32],
    ) {
        for &(neighbor, direction) in &graph[node] {
            if neighbor != parent {
                answer[neighbor] = answer[node] + direction;
                reroot(neighbor, node, graph, answer);
            }
        }
    }

    let mut answer = vec![0; n as usize];
    answer[0] = initial_cost(0, n as usize, &graph);
    reroot(0, n as usize, &graph, &mut answer);
    answer
}

fn main() {
    println!(
        "{:?}",
        min_edge_reversals(4, vec![vec![2, 0], vec![2, 1], vec![1, 3]])
    );
}

#[cfg(test)]
mod tests {
    use super::min_edge_reversals;

    #[test]
    fn example_one() {
        assert_eq!(
            min_edge_reversals(4, vec![vec![2, 0], vec![2, 1], vec![1, 3]]),
            vec![1, 1, 0, 2]
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(
            min_edge_reversals(3, vec![vec![1, 2], vec![2, 0]]),
            vec![2, 0, 1]
        );
    }
}
