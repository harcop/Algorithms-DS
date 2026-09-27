/// LeetCode #3970 - Shortest Path With At Most K Consecutive Identical Characters
use std::cmp::Reverse;
use std::collections::BinaryHeap;

fn shortest_path(n: i32, edges: Vec<Vec<i32>>, labels: String, k: i32) -> i64 {
    let n = n as usize;
    let k = k as usize;
    if n == 1 {
        return 0;
    }
    let labels = labels.into_bytes();
    let mut g = vec![vec![]; n];
    for e in edges {
        g[e[0] as usize].push((e[1] as usize, e[2] as i64));
    }
    let mut dist = vec![vec![i64::MAX; k + 1]; n];
    dist[0][1] = 0;
    let mut pq = BinaryHeap::new();
    pq.push((Reverse(0i64), 0usize, 1usize));
    while let Some((Reverse(d), u, run)) = pq.pop() {
        if d != dist[u][run] {
            continue;
        }
        if u == n - 1 {
            return d;
        }
        for &(v, w) in &g[u] {
            let nr = if labels[v] == labels[u] { run + 1 } else { 1 };
            if nr <= k {
                let nd = d + w;
                if nd < dist[v][nr] {
                    dist[v][nr] = nd;
                    pq.push((Reverse(nd), v, nr));
                }
            }
        }
    }
    -1
}

fn main() {
    println!(
        "{}",
        shortest_path(
            3,
            vec![vec![0, 1, 1], vec![1, 2, 1], vec![0, 2, 3]],
            "aab".into(),
            1
        )
    );
}

#[cfg(test)]
mod tests {
    use super::shortest_path;

    #[test]
    fn example1() {
        assert_eq!(
            shortest_path(
                3,
                vec![vec![0, 1, 1], vec![1, 2, 1], vec![0, 2, 3]],
                "aab".into(),
                1
            ),
            3
        );
    }

    #[test]
    fn example2() {
        assert_eq!(
            shortest_path(
                3,
                vec![vec![0, 1, 1], vec![1, 2, 1], vec![0, 2, 3]],
                "aab".into(),
                2
            ),
            2
        );
    }

    #[test]
    fn example3() {
        assert_eq!(
            shortest_path(3, vec![vec![0, 1, 1], vec![1, 2, 1]], "aaa".into(), 2),
            -1
        );
    }
}
