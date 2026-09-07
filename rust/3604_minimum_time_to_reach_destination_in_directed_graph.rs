/// LeetCode #3604 - Minimum Time to Reach Destination in Directed Graph
use std::cmp::Reverse;
use std::collections::BinaryHeap;

fn min_time(n: i32, edges: Vec<Vec<i32>>) -> i32 {
    let n = n as usize;
    let mut g = vec![Vec::new(); n];
    for e in edges {
        g[e[0] as usize].push((e[1] as usize, e[2], e[3]));
    }
    let mut dist = vec![i32::MAX; n];
    dist[0] = 0;
    let mut heap = BinaryHeap::new();
    heap.push(Reverse((0, 0usize)));
    while let Some(Reverse((t, u))) = heap.pop() {
        if t != dist[u] {
            continue;
        }
        if u == n - 1 {
            return t;
        }
        for &(v, s, e) in &g[u] {
            if t > e {
                continue;
            }
            let nt = t.max(s) + 1;
            if nt < dist[v] {
                dist[v] = nt;
                heap.push(Reverse((nt, v)));
            }
        }
    }
    -1
}

fn main() {
    println!("{}", min_time(3, vec![vec![0, 1, 0, 1], vec![1, 2, 2, 5]]));
}

#[cfg(test)]
mod tests {
    use super::min_time;

    #[test]
    fn example1() {
        assert_eq!(min_time(3, vec![vec![0, 1, 0, 1], vec![1, 2, 2, 5]]), 3);
    }

    #[test]
    fn example2() {
        assert_eq!(
            min_time(
                4,
                vec![
                    vec![0, 1, 0, 3],
                    vec![1, 3, 7, 8],
                    vec![0, 2, 1, 5],
                    vec![2, 3, 4, 7]
                ]
            ),
            5
        );
    }

    #[test]
    fn example3() {
        assert_eq!(min_time(3, vec![vec![1, 0, 1, 3], vec![1, 2, 3, 5]]), -1);
    }

    #[test]
    fn already_there() {
        assert_eq!(min_time(1, vec![]), 0);
    }
}
