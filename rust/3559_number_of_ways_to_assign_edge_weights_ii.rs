/// LeetCode #3559 - Number of Ways to Assign Edge Weights II
use std::collections::VecDeque;

const MOD: i64 = 1_000_000_007;

fn assign_edge_weights(edges: Vec<Vec<i32>>, queries: Vec<Vec<i32>>) -> Vec<i32> {
    let n = edges.len() + 1;
    let m = (usize::BITS - n.leading_zeros()) as usize;
    let mut g = vec![Vec::new(); n + 1];
    for e in &edges {
        let (u, v) = (e[0] as usize, e[1] as usize);
        g[u].push(v);
        g[v].push(u);
    }
    let mut f = vec![vec![0usize; m]; n + 1];
    let mut p = vec![0usize; n + 1];
    let mut depth = vec![0usize; n + 1];
    let mut q = VecDeque::from([1usize]);
    while let Some(i) = q.pop_front() {
        f[i][0] = p[i];
        for j in 1..m {
            f[i][j] = f[f[i][j - 1]][j - 1];
        }
        for &j in &g[i] {
            if j != p[i] {
                p[j] = i;
                depth[j] = depth[i] + 1;
                q.push_back(j);
            }
        }
    }
    let mut pow2 = vec![1i64; n];
    for i in 1..n {
        pow2[i] = pow2[i - 1] * 2 % MOD;
    }
    queries
        .into_iter()
        .map(|qq| {
            let (u, v) = (qq[0] as usize, qq[1] as usize);
            let mut x = u;
            let mut y = v;
            if depth[x] < depth[y] {
                std::mem::swap(&mut x, &mut y);
            }
            for j in (0..m).rev() {
                if depth[x] - depth[y] >= (1 << j) {
                    x = f[x][j];
                }
            }
            for j in (0..m).rev() {
                if f[x][j] != f[y][j] {
                    x = f[x][j];
                    y = f[y][j];
                }
            }
            if x != y {
                x = p[x];
            }
            let d = depth[u] + depth[v] - 2 * depth[x];
            if d == 0 {
                0
            } else {
                pow2[d - 1] as i32
            }
        })
        .collect()
}

fn main() {
    println!("{:?}", assign_edge_weights(vec![vec![1, 2]], vec![vec![1, 1], vec![1, 2]]));
}

#[cfg(test)]
mod tests {
    use super::assign_edge_weights;

    #[test]
    fn example1() {
        assert_eq!(
            assign_edge_weights(vec![vec![1, 2]], vec![vec![1, 1], vec![1, 2]]),
            vec![0, 1]
        );
    }

    #[test]
    fn example2() {
        assert_eq!(
            assign_edge_weights(
                vec![vec![1, 2], vec![1, 3], vec![3, 4], vec![3, 5]],
                vec![vec![1, 4], vec![3, 4], vec![2, 5]]
            ),
            vec![2, 1, 4]
        );
    }
}
