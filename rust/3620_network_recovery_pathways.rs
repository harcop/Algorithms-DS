/// LeetCode #3620 - Network Recovery Pathways
use std::cmp::Reverse;
use std::collections::BinaryHeap;

fn find_max_path_score(edges: Vec<Vec<i32>>, online: Vec<bool>, k: i64) -> i32 {
    let n = online.len();
    let mut g = vec![Vec::new(); n];
    let mut l = i32::MAX;
    let mut r = 0;
    for e in &edges {
        let (u, v, w) = (e[0] as usize, e[1] as usize, e[2]);
        if !online[u] || !online[v] {
            continue;
        }
        g[u].push((v, w));
        l = l.min(w);
        r = r.max(w);
    }
    if l == i32::MAX {
        return -1;
    }
    let check = |mid: i32| -> bool {
        let inf = k.saturating_add(1).max(1);
        let mut dist = vec![inf; n];
        dist[0] = 0;
        let mut pq = BinaryHeap::new();
        pq.push(Reverse((0i64, 0usize)));
        while let Some(Reverse((d, u))) = pq.pop() {
            if d > k {
                return false;
            }
            if u == n - 1 {
                return true;
            }
            if dist[u] < d {
                continue;
            }
            for &(v, w) in &g[u] {
                if w < mid {
                    continue;
                }
                let nd = d + w as i64;
                if nd < dist[v] {
                    dist[v] = nd;
                    pq.push(Reverse((nd, v)));
                }
            }
        }
        false
    };
    while l < r {
        let mid = (l + r + 1) >> 1;
        if check(mid) {
            l = mid;
        } else {
            r = mid - 1;
        }
    }
    if check(l) {
        l
    } else {
        -1
    }
}

fn main() {
    println!(
        "{}",
        find_max_path_score(
            vec![vec![0, 1, 5], vec![1, 3, 10], vec![0, 2, 3], vec![2, 3, 4]],
            vec![true, true, true, true],
            10
        )
    );
}

#[cfg(test)]
mod tests {
    use super::find_max_path_score;

    #[test]
    fn example1() {
        assert_eq!(
            find_max_path_score(
                vec![vec![0, 1, 5], vec![1, 3, 10], vec![0, 2, 3], vec![2, 3, 4]],
                vec![true, true, true, true],
                10
            ),
            3
        );
    }

    #[test]
    fn example2() {
        assert_eq!(
            find_max_path_score(
                vec![
                    vec![0, 1, 7],
                    vec![1, 4, 5],
                    vec![0, 2, 6],
                    vec![2, 3, 6],
                    vec![3, 4, 2],
                    vec![2, 4, 6]
                ],
                vec![true, true, true, false, true],
                12
            ),
            6
        );
    }
}
