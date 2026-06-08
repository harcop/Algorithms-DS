/// LeetCode #1786 - Number of Restricted Paths From First to Last Node
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::HashMap;

const MOD: i64 = 1_000_000_007;

fn count_restricted_paths(n: i32, edges: Vec<Vec<i32>>) -> i32 {
    let n = n as usize;
    let mut g: Vec<Vec<(usize, i64)>> = vec![Vec::new(); n + 1];
    for e in edges {
        let u = e[0] as usize;
        let v = e[1] as usize;
        let w = e[2] as i64;
        g[u].push((v, w));
        g[v].push((u, w));
    }

    let mut dist = vec![i64::MAX; n + 1];
    dist[n] = 0;
    let mut heap = BinaryHeap::new();
    heap.push(Reverse((0i64, n)));

    while let Some(Reverse((d, u))) = heap.pop() {
        if d > dist[u] {
            continue;
        }
        for &(v, w) in &g[u] {
            let nd = d + w;
            if nd < dist[v] {
                dist[v] = nd;
                heap.push(Reverse((nd, v)));
            }
        }
    }

    let mut memo: HashMap<usize, i64> = HashMap::new();
    fn dfs(
        u: usize,
        n: usize,
        g: &Vec<Vec<(usize, i64)>>,
        dist: &[i64],
        memo: &mut HashMap<usize, i64>,
    ) -> i64 {
        if u == n {
            return 1;
        }
        if let Some(&v) = memo.get(&u) {
            return v;
        }
        let mut ans = 0i64;
        for &(v, _) in &g[u] {
            if dist[u] > dist[v] {
                ans = (ans + dfs(v, n, g, dist, memo)) % MOD;
            }
        }
        memo.insert(u, ans);
        ans
    }

    dfs(1, n, &g, &dist, &mut memo) as i32
}

fn main() {
    println!(
        "{}",
        count_restricted_paths(
            5,
            vec![
                vec![1, 2, 3],
                vec![1, 3, 3],
                vec![2, 3, 1],
                vec![1, 4, 2],
                vec![5, 2, 2],
                vec![3, 5, 1],
                vec![5, 4, 10],
            ],
        )
    );
}

#[cfg(test)]
mod tests {
    use super::count_restricted_paths;

    #[test]
    fn example_one() {
        assert_eq!(
            count_restricted_paths(
                5,
                vec![
                    vec![1, 2, 3],
                    vec![1, 3, 3],
                    vec![2, 3, 1],
                    vec![1, 4, 2],
                    vec![5, 2, 2],
                    vec![3, 5, 1],
                    vec![5, 4, 10],
                ],
            ),
            3
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(
            count_restricted_paths(
                7,
                vec![
                    vec![1, 3, 1],
                    vec![4, 1, 2],
                    vec![7, 3, 4],
                    vec![2, 5, 3],
                    vec![5, 6, 1],
                    vec![6, 7, 2],
                    vec![7, 5, 3],
                    vec![2, 6, 4],
                ],
            ),
            1
        );
    }
}
