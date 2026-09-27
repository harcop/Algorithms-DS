/// LeetCode #3977 - Minimum Time to Reach Target With Limited Power
use std::cmp::Reverse;
use std::collections::BinaryHeap;

fn min_time_max_power(
    n: i32,
    edges: Vec<Vec<i32>>,
    power: i32,
    cost: Vec<i32>,
    source: i32,
    target: i32,
) -> Vec<i64> {
    let n = n as usize;
    let power = power as usize;
    let source = source as usize;
    let target = target as usize;
    let mut g = vec![vec![]; n];
    for e in edges {
        g[e[0] as usize].push((e[1] as usize, e[2] as i64));
    }
    let mut dist = vec![vec![i64::MAX; power + 1]; n];
    dist[source][power] = 0;
    let mut pq: BinaryHeap<(Reverse<i64>, i32, usize)> = BinaryHeap::new();
    pq.push((Reverse(0), power as i32, source));
    while let Some((Reverse(d), p, u)) = pq.pop() {
        let p = p as usize;
        if u == target {
            return vec![d, p as i64];
        }
        if d > dist[u][p] || (p as i32) < cost[u] {
            continue;
        }
        let np = p as i32 - cost[u];
        for &(v, t) in &g[u] {
            let nd = d + t;
            if nd < dist[v][np as usize] {
                dist[v][np as usize] = nd;
                pq.push((Reverse(nd), np, v));
            }
        }
    }
    vec![-1, -1]
}

fn main() {
    println!(
        "{:?}",
        min_time_max_power(
            5,
            vec![
                vec![0, 1, 1],
                vec![1, 4, 1],
                vec![0, 2, 1],
                vec![2, 3, 1],
                vec![3, 4, 1]
            ],
            4,
            vec![2, 3, 1, 1, 1],
            0,
            4
        )
    );
}

#[cfg(test)]
mod tests {
    use super::min_time_max_power;

    #[test]
    fn example1() {
        assert_eq!(
            min_time_max_power(
                5,
                vec![
                    vec![0, 1, 1],
                    vec![1, 4, 1],
                    vec![0, 2, 1],
                    vec![2, 3, 1],
                    vec![3, 4, 1]
                ],
                4,
                vec![2, 3, 1, 1, 1],
                0,
                4
            ),
            vec![3, 0]
        );
    }

    #[test]
    fn example2() {
        assert_eq!(
            min_time_max_power(
                3,
                vec![vec![0, 1, 2], vec![1, 2, 2], vec![2, 0, 2]],
                3,
                vec![1, 1, 1],
                1,
                1
            ),
            vec![0, 3]
        );
    }

    #[test]
    fn example3() {
        assert_eq!(
            min_time_max_power(4, vec![vec![0, 1, 3], vec![2, 3, 4]], 3, vec![1, 1, 1, 1], 0, 3),
            vec![-1, -1]
        );
    }
}
