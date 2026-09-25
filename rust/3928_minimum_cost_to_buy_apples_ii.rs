/// LeetCode #3928 - Minimum Cost to Buy Apples II
use std::cmp::Reverse;
use std::collections::BinaryHeap;

fn minimum_cost(n: i32, prices: Vec<i32>, roads: Vec<Vec<i32>>) -> Vec<i64> {
    let n = n as usize;
    let mut empty = vec![Vec::new(); n];
    let mut loaded = vec![Vec::new(); n];
    for r in roads {
        let u = r[0] as usize;
        let v = r[1] as usize;
        let cost = r[2] as i64;
        let tax = r[3] as i64;
        empty[u].push((v, cost));
        empty[v].push((u, cost));
        let back = cost * tax;
        loaded[u].push((v, back));
        loaded[v].push((u, back));
    }
    let shortest = |g: &[Vec<(usize, i64)>], src: usize| -> Vec<i64> {
        let mut dist = vec![i64::MAX; n];
        dist[src] = 0;
        let mut pq = BinaryHeap::new();
        pq.push((Reverse(0i64), src));
        while let Some((Reverse(d), u)) = pq.pop() {
            if d != dist[u] {
                continue;
            }
            for &(v, w) in &g[u] {
                let nd = d + w;
                if nd < dist[v] {
                    dist[v] = nd;
                    pq.push((Reverse(nd), v));
                }
            }
        }
        dist
    };
    let mut ans = vec![0i64; n];
    for i in 0..n {
        let de = shortest(&empty, i);
        let dl = shortest(&loaded, i);
        let mut best = prices[i] as i64;
        for j in 0..n {
            if de[j] == i64::MAX || dl[j] == i64::MAX {
                continue;
            }
            best = best.min(de[j] + dl[j] + prices[j] as i64);
        }
        ans[i] = best;
    }
    ans
}

fn main() {
    println!(
        "{:?}",
        minimum_cost(2, vec![8, 3], vec![vec![0, 1, 1, 2]])
    );
}

#[cfg(test)]
mod tests {
    use super::minimum_cost;

    #[test]
    fn example1() {
        assert_eq!(
            minimum_cost(2, vec![8, 3], vec![vec![0, 1, 1, 2]]),
            vec![6, 3]
        );
    }

    #[test]
    fn example2() {
        assert_eq!(
            minimum_cost(3, vec![9, 4, 6], vec![vec![0, 1, 1, 3], vec![1, 2, 4, 2]]),
            vec![8, 4, 6]
        );
    }

    #[test]
    fn example3() {
        assert_eq!(
            minimum_cost(
                3,
                vec![10, 11, 1],
                vec![vec![0, 2, 1, 3], vec![1, 2, 3, 4], vec![0, 1, 5, 2]]
            ),
            vec![5, 11, 1]
        );
    }
}
