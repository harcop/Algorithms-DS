/// LeetCode #3650 - Minimum Cost Path with Edge Reversals
use std::cmp::Reverse;
use std::collections::BinaryHeap;

fn min_cost(n: i32, edges: Vec<Vec<i32>>) -> i32 {
    let n = n as usize;
    let mut g = vec![Vec::new(); n];
    for e in edges {
        let (u, v, w) = (e[0] as usize, e[1] as usize, e[2]);
        g[u].push((v, w));
        g[v].push((u, w * 2));
    }
    let inf = i32::MAX / 2;
    let mut dist = vec![inf; n];
    dist[0] = 0;
    let mut pq = BinaryHeap::new();
    pq.push(Reverse((0, 0usize)));
    while let Some(Reverse((d, u))) = pq.pop() {
        if d > dist[u] {
            continue;
        }
        if u == n - 1 {
            return d;
        }
        for &(v, w) in &g[u] {
            let nd = d + w;
            if nd < dist[v] {
                dist[v] = nd;
                pq.push(Reverse((nd, v)));
            }
        }
    }
    -1
}

fn main() {
    println!("{}", min_cost(4, vec![vec![0,1,3],vec![3,1,1],vec![2,3,4],vec![0,2,2]]));
}

#[cfg(test)]
mod tests {
    use super::min_cost;

    #[test]
    fn example1() {
        assert_eq!(
            min_cost(4, vec![vec![0,1,3],vec![3,1,1],vec![2,3,4],vec![0,2,2]]),
            5
        );
    }

    #[test]
    fn example2() {
        assert_eq!(
            min_cost(4, vec![vec![0,2,1],vec![2,1,1],vec![1,3,1],vec![2,3,3]]),
            3
        );
    }
}
