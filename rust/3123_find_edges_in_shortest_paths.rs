/// LeetCode #3123 - Find Edges in Shortest Paths
use std::cmp::Reverse;
use std::collections::{BinaryHeap, VecDeque};

fn find_answer(n: i32, edges: Vec<Vec<i32>>) -> Vec<bool> {
    let n = n as usize;
    let m = edges.len();
    let mut g = vec![Vec::new(); n];
    for (i, e) in edges.iter().enumerate() {
        let (a, b, w) = (e[0] as usize, e[1] as usize, e[2]);
        g[a].push((b, w, i));
        g[b].push((a, w, i));
    }
    const INF: i32 = 1 << 30;
    let mut dist = vec![INF; n];
    dist[0] = 0;
    let mut pq = BinaryHeap::new();
    pq.push(Reverse((0, 0usize)));
    while let Some(Reverse((da, a))) = pq.pop() {
        if da > dist[a] {
            continue;
        }
        for &(b, w, _) in &g[a] {
            if dist[b] > dist[a] + w {
                dist[b] = dist[a] + w;
                pq.push(Reverse((dist[b], b)));
            }
        }
    }
    let mut ans = vec![false; m];
    if dist[n - 1] == INF {
        return ans;
    }
    let mut q = VecDeque::new();
    q.push_back(n - 1);
    let mut seen = vec![false; n];
    seen[n - 1] = true;
    while let Some(a) = q.pop_front() {
        for &(b, w, i) in &g[a] {
            if dist[a] == dist[b] + w {
                ans[i] = true;
                if !seen[b] {
                    seen[b] = true;
                    q.push_back(b);
                }
            }
        }
    }
    ans
}

fn main() {
    println!(
        "{:?}",
        find_answer(
            6,
            vec![
                vec![0, 1, 4],
                vec![0, 2, 1],
                vec![1, 3, 2],
                vec![1, 4, 3],
                vec![1, 5, 1],
                vec![2, 3, 1],
                vec![3, 5, 3],
                vec![4, 5, 2]
            ]
        )
    );
}

#[cfg(test)]
mod tests {
    use super::find_answer;

    #[test]
    fn example1() {
        assert_eq!(
            find_answer(
                6,
                vec![
                    vec![0, 1, 4],
                    vec![0, 2, 1],
                    vec![1, 3, 2],
                    vec![1, 4, 3],
                    vec![1, 5, 1],
                    vec![2, 3, 1],
                    vec![3, 5, 3],
                    vec![4, 5, 2]
                ]
            ),
            vec![true, true, true, false, true, true, true, false]
        );
    }

    #[test]
    fn example2() {
        assert_eq!(
            find_answer(
                4,
                vec![vec![2, 0, 1], vec![0, 1, 1], vec![0, 3, 4], vec![3, 2, 2]]
            ),
            vec![true, false, false, true]
        );
    }
}
