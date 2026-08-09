/// LeetCode #3112 - Minimum Time to Visit Disappearing Nodes
use std::cmp::Reverse;
use std::collections::BinaryHeap;

fn minimum_time(n: i32, edges: Vec<Vec<i32>>, disappear: Vec<i32>) -> Vec<i32> {
    let n = n as usize;
    let mut g = vec![Vec::new(); n];
    for e in edges {
        let (u, v, w) = (e[0] as usize, e[1] as usize, e[2]);
        g[u].push((v, w));
        g[v].push((u, w));
    }
    let mut dist = vec![i32::MAX / 2; n];
    dist[0] = 0;
    let mut pq = BinaryHeap::new();
    pq.push(Reverse((0, 0usize)));
    while let Some(Reverse((du, u))) = pq.pop() {
        if du > dist[u] {
            continue;
        }
        for &(v, w) in &g[u] {
            let nd = dist[u] + w;
            if dist[v] > nd && nd < disappear[v] {
                dist[v] = nd;
                pq.push(Reverse((nd, v)));
            }
        }
    }
    dist.into_iter()
        .zip(disappear)
        .map(|(a, b)| if a < b { a } else { -1 })
        .collect()
}

fn main() {
    println!(
        "{:?}",
        minimum_time(
            3,
            vec![vec![0, 1, 2], vec![1, 2, 1], vec![0, 2, 4]],
            vec![1, 1, 5]
        )
    );
}

#[cfg(test)]
mod tests {
    use super::minimum_time;

    #[test]
    fn example1() {
        assert_eq!(
            minimum_time(
                3,
                vec![vec![0, 1, 2], vec![1, 2, 1], vec![0, 2, 4]],
                vec![1, 1, 5]
            ),
            vec![0, -1, 4]
        );
    }

    #[test]
    fn example2() {
        assert_eq!(
            minimum_time(
                3,
                vec![vec![0, 1, 2], vec![1, 2, 1], vec![0, 2, 4]],
                vec![1, 3, 5]
            ),
            vec![0, 2, 3]
        );
    }

    #[test]
    fn example3() {
        assert_eq!(
            minimum_time(2, vec![vec![0, 1, 1]], vec![1, 1]),
            vec![0, -1]
        );
    }
}
