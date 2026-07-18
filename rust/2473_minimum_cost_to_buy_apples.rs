/// LeetCode #2473 - Minimum Cost to Buy Apples
use std::cmp::Reverse;
use std::collections::BinaryHeap;

fn min_cost(n: i32, roads: Vec<Vec<i32>>, apple_cost: Vec<i32>, k: i32) -> Vec<i64> {
    let n = n as usize;
    let mut graph = vec![Vec::new(); n];
    for road in roads {
        let u = (road[0] - 1) as usize;
        let v = (road[1] - 1) as usize;
        let w = road[2] as i64;
        graph[u].push((v, w));
        graph[v].push((u, w));
    }

    (0..n)
        .map(|src| dijkstra(&graph, src, &apple_cost, k as i64))
        .collect()
}

fn dijkstra(graph: &[Vec<(usize, i64)>], src: usize, apple_cost: &[i32], k: i64) -> i64 {
    let mut dist = vec![i64::MAX; graph.len()];
    dist[src] = 0;
    let mut heap = BinaryHeap::from([Reverse((0i64, src))]);
    let mut answer = i64::MAX;

    while let Some(Reverse((d, u))) = heap.pop() {
        if d > dist[u] {
            continue;
        }
        answer = answer.min(apple_cost[u] as i64 + (k + 1) * d);
        for &(v, w) in &graph[u] {
            let next = d + w;
            if next < dist[v] {
                dist[v] = next;
                heap.push(Reverse((next, v)));
            }
        }
    }

    answer
}

fn main() {
    println!(
        "{:?}",
        min_cost(
            4,
            vec![
                vec![1, 2, 4],
                vec![2, 3, 2],
                vec![2, 4, 5],
                vec![3, 4, 1],
                vec![1, 3, 4]
            ],
            vec![56, 42, 102, 301],
            2
        )
    );
}

#[cfg(test)]
mod tests {
    use super::min_cost;

    #[test]
    fn example_one() {
        assert_eq!(
            min_cost(
                4,
                vec![
                    vec![1, 2, 4],
                    vec![2, 3, 2],
                    vec![2, 4, 5],
                    vec![3, 4, 1],
                    vec![1, 3, 4]
                ],
                vec![56, 42, 102, 301],
                2
            ),
            vec![54, 42, 48, 51]
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(
            min_cost(
                3,
                vec![vec![1, 2, 5], vec![2, 3, 1], vec![3, 1, 2]],
                vec![2, 3, 1],
                3
            ),
            vec![2, 3, 1]
        );
    }
}
