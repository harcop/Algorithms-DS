/// LeetCode #3778 - Minimum Distance Excluding One Maximum Weighted Edge (premium)
use std::cmp::Reverse;
use std::collections::BinaryHeap;

fn min_cost_excluding_max(n: i32, edges: Vec<Vec<i32>>) -> i64 {
    let n = n as usize;
    let mut g = vec![Vec::new(); n];
    for e in edges {
        let u = e[0] as usize;
        let v = e[1] as usize;
        let w = e[2] as i64;
        g[u].push((v, w));
        g[v].push((u, w));
    }
    const INF: i64 = i64::MAX / 4;
    let mut dist = vec![[INF; 2]; n];
    dist[0][0] = 0;
    let mut pq = BinaryHeap::new();
    pq.push(Reverse((0i64, 0usize, 0usize)));
    while let Some(Reverse((cur, u, used))) = pq.pop() {
        if cur > dist[u][used] {
            continue;
        }
        if u == n - 1 && used == 1 {
            return cur;
        }
        for &(v, w) in &g[u] {
            let nxt = cur + w;
            if nxt < dist[v][used] {
                dist[v][used] = nxt;
                pq.push(Reverse((nxt, v, used)));
            }
            if used == 0 && cur < dist[v][1] {
                dist[v][1] = cur;
                pq.push(Reverse((cur, v, 1)));
            }
        }
    }
    dist[n - 1][1]
}

fn main() {
    println!(
        "{}",
        min_cost_excluding_max(
            5,
            vec![vec![0, 1, 2], vec![1, 2, 7], vec![2, 3, 7], vec![3, 4, 4]]
        )
    );
}

#[cfg(test)]
mod tests {
    use super::min_cost_excluding_max;

    #[test]
    fn example1() {
        assert_eq!(
            min_cost_excluding_max(
                5,
                vec![vec![0, 1, 2], vec![1, 2, 7], vec![2, 3, 7], vec![3, 4, 4]]
            ),
            13
        );
    }

    #[test]
    fn example2() {
        assert_eq!(
            min_cost_excluding_max(3, vec![vec![0, 1, 1], vec![1, 2, 1], vec![0, 2, 50000]]),
            0
        );
    }
}
