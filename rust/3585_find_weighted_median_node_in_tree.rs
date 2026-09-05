/// LeetCode #3585 - Find Weighted Median Node in Tree
use std::collections::VecDeque;

fn find_median(n: i32, edges: Vec<Vec<i32>>, queries: Vec<Vec<i32>>) -> Vec<i32> {
    let n = n as usize;
    let log = (usize::BITS - n.leading_zeros()) as usize + 1;
    let mut g = vec![Vec::new(); n];
    for e in &edges {
        let (u, v, w) = (e[0] as usize, e[1] as usize, e[2] as i64);
        g[u].push((v, w));
        g[v].push((u, w));
    }
    let mut parent = vec![vec![0usize; log]; n];
    let mut depth = vec![0usize; n];
    let mut dist = vec![0i64; n];
    let mut q = VecDeque::from([0usize]);
    let mut vis = vec![false; n];
    vis[0] = true;
    while let Some(u) = q.pop_front() {
        for &(v, w) in &g[u] {
            if vis[v] {
                continue;
            }
            vis[v] = true;
            parent[v][0] = u;
            depth[v] = depth[u] + 1;
            dist[v] = dist[u] + w;
            q.push_back(v);
        }
    }
    for j in 1..log {
        for i in 0..n {
            parent[i][j] = parent[parent[i][j - 1]][j - 1];
        }
    }
    let lca = |mut u: usize, mut v: usize| -> usize {
        if depth[u] < depth[v] {
            std::mem::swap(&mut u, &mut v);
        }
        let mut diff = depth[u] - depth[v];
        for j in 0..log {
            if (diff >> j) & 1 == 1 {
                u = parent[u][j];
            }
        }
        if u == v {
            return u;
        }
        for j in (0..log).rev() {
            if parent[u][j] != parent[v][j] {
                u = parent[u][j];
                v = parent[v][j];
            }
        }
        parent[u][0]
    };
    queries
        .into_iter()
        .map(|qq| {
            let (u, v) = (qq[0] as usize, qq[1] as usize);
            if u == v {
                return u as i32;
            }
            let lc = lca(u, v);
            let w = dist[u] + dist[v] - 2 * dist[lc];
            let d_u_lc = dist[u] - dist[lc];
            if 2 * d_u_lc >= w {
                let mut x = u;
                for j in (0..log).rev() {
                    let p = parent[x][j];
                    if depth[p] >= depth[lc] && 2 * (dist[u] - dist[p]) < w {
                        x = p;
                    }
                }
                parent[x][0] as i32
            } else {
                let mut x = v;
                for j in (0..log).rev() {
                    let p = parent[x][j];
                    if depth[p] >= depth[lc] {
                        let d = dist[u] + dist[p] - 2 * dist[lc];
                        if 2 * d >= w {
                            x = p;
                        }
                    }
                }
                x as i32
            }
        })
        .collect()
}

fn main() {
    println!("{:?}", find_median(2, vec![vec![0, 1, 7]], vec![vec![1, 0], vec![0, 1]]));
}

#[cfg(test)]
mod tests {
    use super::find_median;

    #[test]
    fn example1() {
        assert_eq!(
            find_median(2, vec![vec![0, 1, 7]], vec![vec![1, 0], vec![0, 1]]),
            vec![0, 1]
        );
    }

    #[test]
    fn example2() {
        assert_eq!(
            find_median(
                3,
                vec![vec![0, 1, 2], vec![2, 0, 4]],
                vec![vec![0, 1], vec![2, 0], vec![1, 2]]
            ),
            vec![1, 0, 2]
        );
    }

    #[test]
    fn example3() {
        assert_eq!(
            find_median(
                5,
                vec![vec![0, 1, 2], vec![0, 2, 5], vec![1, 3, 1], vec![2, 4, 3]],
                vec![vec![3, 4], vec![1, 2]]
            ),
            vec![2, 2]
        );
    }
}
