/// LeetCode #2714 - Find Shortest Path with K Hops
use std::cmp::Reverse;
use std::collections::BinaryHeap;

fn shortest_path_with_hops(
    n: i32,
    edges: Vec<Vec<i32>>,
    s: i32,
    d: i32,
    k: i32,
) -> i32 {
    let n = n as usize;
    let s = s as usize;
    let d = d as usize;
    let k = k as usize;
    let mut g = vec![Vec::new(); n];
    for e in &edges {
        let u = e[0] as usize;
        let v = e[1] as usize;
        let w = e[2];
        g[u].push((v, w));
        g[v].push((u, w));
    }
    const INF: i32 = i32::MAX / 4;
    let mut dist = vec![vec![INF; k + 1]; n];
    dist[s][0] = 0;
    let mut pq = BinaryHeap::new();
    pq.push(Reverse((0, s, 0usize)));
    while let Some(Reverse((dis, u, t))) = pq.pop() {
        if dis > dist[u][t] {
            continue;
        }
        for &(v, w) in &g[u] {
            if t + 1 <= k && dist[v][t + 1] > dis {
                dist[v][t + 1] = dis;
                pq.push(Reverse((dis, v, t + 1)));
            }
            if dist[v][t] > dis + w {
                dist[v][t] = dis + w;
                pq.push(Reverse((dis + w, v, t)));
            }
        }
    }
    *dist[d].iter().min().unwrap()
}

fn main() {
    println!(
        "{}",
        shortest_path_with_hops(4, vec![vec![0, 1, 4], vec![0, 2, 2], vec![2, 3, 6]], 1, 3, 2)
    );
}

#[cfg(test)]
mod tests {
    use super::shortest_path_with_hops;

    #[test]
    fn example_one() {
        assert_eq!(
            shortest_path_with_hops(
                4,
                vec![vec![0, 1, 4], vec![0, 2, 2], vec![2, 3, 6]],
                1,
                3,
                2
            ),
            2
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(
            shortest_path_with_hops(
                7,
                vec![
                    vec![3, 1, 9],
                    vec![3, 2, 4],
                    vec![4, 0, 9],
                    vec![0, 5, 6],
                    vec![3, 6, 2],
                    vec![6, 0, 4],
                    vec![1, 2, 4],
                ],
                4,
                1,
                2
            ),
            6
        );
    }

    #[test]
    fn example_three() {
        assert_eq!(
            shortest_path_with_hops(
                5,
                vec![
                    vec![0, 4, 2],
                    vec![0, 1, 3],
                    vec![0, 2, 1],
                    vec![2, 1, 4],
                    vec![1, 3, 4],
                    vec![3, 4, 7],
                ],
                2,
                3,
                1
            ),
            3
        );
    }
}
