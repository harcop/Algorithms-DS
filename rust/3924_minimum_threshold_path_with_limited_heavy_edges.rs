/// LeetCode #3924 - Minimum Threshold Path With Limited Heavy Edges
use std::collections::VecDeque;

fn minimum_threshold(n: i32, edges: Vec<Vec<i32>>, source: i32, target: i32, k: i32) -> i32 {
    if source == target {
        return 0;
    }
    let n = n as usize;
    let source = source as usize;
    let target = target as usize;
    let mut g = vec![Vec::new(); n];
    let mut hi = 0i32;
    for e in &edges {
        let u = e[0] as usize;
        let v = e[1] as usize;
        let w = e[2];
        g[u].push((v, w));
        g[v].push((u, w));
        hi = hi.max(w);
    }
    if edges.is_empty() {
        return -1;
    }
    let ok = |t: i32| -> bool {
        let mut dist = vec![i32::MAX; n];
        let mut dq = VecDeque::new();
        dist[source] = 0;
        dq.push_back(source);
        while let Some(u) = dq.pop_front() {
            let d = dist[u];
            if d > k {
                continue;
            }
            for &(v, w) in &g[u] {
                let add = if w <= t { 0 } else { 1 };
                let nd = d + add;
                if nd < dist[v] {
                    dist[v] = nd;
                    if add == 0 {
                        dq.push_front(v);
                    } else {
                        dq.push_back(v);
                    }
                }
            }
        }
        dist[target] <= k
    };
    if !ok(hi) {
        return -1;
    }
    let mut lo = 0i32;
    while lo < hi {
        let mid = lo + (hi - lo) / 2;
        if ok(mid) {
            hi = mid;
        } else {
            lo = mid + 1;
        }
    }
    lo
}

fn main() {
    println!(
        "{}",
        minimum_threshold(
            6,
            vec![
                vec![0, 1, 5],
                vec![1, 2, 3],
                vec![3, 4, 4],
                vec![4, 5, 1],
                vec![1, 4, 2]
            ],
            0,
            3,
            1
        )
    );
}

#[cfg(test)]
mod tests {
    use super::minimum_threshold;

    #[test]
    fn example1() {
        assert_eq!(
            minimum_threshold(
                6,
                vec![
                    vec![0, 1, 5],
                    vec![1, 2, 3],
                    vec![3, 4, 4],
                    vec![4, 5, 1],
                    vec![1, 4, 2]
                ],
                0,
                3,
                1
            ),
            4
        );
    }

    #[test]
    fn example2() {
        assert_eq!(
            minimum_threshold(
                6,
                vec![vec![0, 1, 3], vec![1, 2, 4], vec![3, 4, 5], vec![4, 5, 6]],
                0,
                4,
                1
            ),
            -1
        );
    }

    #[test]
    fn example3() {
        assert_eq!(
            minimum_threshold(
                4,
                vec![vec![0, 1, 2], vec![1, 2, 2], vec![2, 3, 2], vec![3, 0, 2]],
                0,
                0,
                0
            ),
            0
        );
    }
}
