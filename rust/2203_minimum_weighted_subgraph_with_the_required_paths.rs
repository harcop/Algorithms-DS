/// LeetCode #2203 - Minimum Weighted Subgraph With the Required Paths
use std::cmp::Reverse;
use std::collections::BinaryHeap;

const MAX: i64 = 10_000_000_000;

fn dijkstra(graph: &[Vec<(usize, i32)>], src: usize) -> Vec<i64> {
    let n = graph.len();
    let mut dist = vec![MAX; n];
    dist[src] = 0;
    let mut heap = BinaryHeap::new();
    heap.push(Reverse((0i64, src)));

    while let Some(Reverse((d, u))) = heap.pop() {
        if d > dist[u] {
            continue;
        }
        for &(v, w) in &graph[u] {
            let nd = d + w as i64;
            if nd < dist[v] {
                dist[v] = nd;
                heap.push(Reverse((nd, v)));
            }
        }
    }

    dist
}

fn minimum_weight(
    n: i32,
    edges: Vec<Vec<i32>>,
    src1: i32,
    src2: i32,
    dest: i32,
) -> i64 {
    let n = n as usize;
    let mut graph = vec![vec![]; n];
    let mut reversed = vec![vec![]; n];

    for e in edges {
        let u = e[0] as usize;
        let v = e[1] as usize;
        let w = e[2];
        graph[u].push((v, w));
        reversed[v].push((u, w));
    }

    let from_src1 = dijkstra(&graph, src1 as usize);
    let from_src2 = dijkstra(&graph, src2 as usize);
    let from_dest = dijkstra(&reversed, dest as usize);

    let mut ans = MAX;
    for i in 0..n {
        if from_src1[i] == MAX || from_src2[i] == MAX || from_dest[i] == MAX {
            continue;
        }
        ans = ans.min(from_src1[i] + from_src2[i] + from_dest[i]);
    }

    if ans == MAX { -1 } else { ans }
}

fn main() {
    println!(
        "{}",
        minimum_weight(
            6,
            vec![
                vec![0, 2, 2],
                vec![0, 5, 6],
                vec![1, 0, 3],
                vec![1, 4, 5],
                vec![2, 1, 1],
                vec![2, 3, 3],
                vec![2, 3, 4],
                vec![3, 4, 2],
                vec![4, 5, 1],
            ],
            0,
            1,
            5,
        )
    );
}

#[cfg(test)]
mod tests {
    use super::minimum_weight;

    #[test]
    fn example_one() {
        assert_eq!(
            minimum_weight(
                6,
                vec![
                    vec![0, 2, 2],
                    vec![0, 5, 6],
                    vec![1, 0, 3],
                    vec![1, 4, 5],
                    vec![2, 1, 1],
                    vec![2, 3, 3],
                    vec![2, 3, 4],
                    vec![3, 4, 2],
                    vec![4, 5, 1],
                ],
                0,
                1,
                5,
            ),
            9
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(
            minimum_weight(3, vec![vec![0, 1, 1], vec![2, 1, 1]], 0, 1, 2),
            -1
        );
    }
}
