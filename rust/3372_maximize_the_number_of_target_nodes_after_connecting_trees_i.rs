/// LeetCode #3372 - Maximize the Number of Target Nodes After Connecting Trees I
fn max_target_nodes(edges1: Vec<Vec<i32>>, edges2: Vec<Vec<i32>>, k: i32) -> Vec<i32> {
    fn build(edges: &[Vec<i32>]) -> Vec<Vec<usize>> {
        let n = edges.len() + 1;
        let mut g = vec![vec![]; n];
        for e in edges {
            let a = e[0] as usize;
            let b = e[1] as usize;
            g[a].push(b);
            g[b].push(a);
        }
        g
    }
    fn dfs(g: &[Vec<usize>], a: usize, fa: i32, d: i32) -> i32 {
        if d < 0 {
            return 0;
        }
        let mut cnt = 1;
        for &b in &g[a] {
            if b as i32 != fa {
                cnt += dfs(g, b, a as i32, d - 1);
            }
        }
        cnt
    }
    let g2 = build(&edges2);
    let m = g2.len();
    let t = (0..m)
        .map(|i| dfs(&g2, i, -1, k - 1))
        .max()
        .unwrap_or(0);
    let g1 = build(&edges1);
    let n = g1.len();
    (0..n).map(|i| dfs(&g1, i, -1, k) + t).collect()
}

fn main() {
    println!(
        "{:?}",
        max_target_nodes(
            vec![vec![0, 1], vec![0, 2], vec![2, 3], vec![2, 4]],
            vec![
                vec![0, 1],
                vec![0, 2],
                vec![0, 3],
                vec![2, 7],
                vec![1, 4],
                vec![4, 5],
                vec![4, 6]
            ],
            2
        )
    );
}

#[cfg(test)]
mod tests {
    use super::max_target_nodes;

    #[test]
    fn example1() {
        assert_eq!(
            max_target_nodes(
                vec![vec![0, 1], vec![0, 2], vec![2, 3], vec![2, 4]],
                vec![
                    vec![0, 1],
                    vec![0, 2],
                    vec![0, 3],
                    vec![2, 7],
                    vec![1, 4],
                    vec![4, 5],
                    vec![4, 6]
                ],
                2
            ),
            vec![9, 7, 9, 8, 8]
        );
    }

    #[test]
    fn example2() {
        assert_eq!(
            max_target_nodes(
                vec![vec![0, 1], vec![0, 2], vec![0, 3], vec![0, 4]],
                vec![vec![0, 1], vec![1, 2], vec![2, 3]],
                1
            ),
            vec![6, 3, 3, 3, 3]
        );
    }
}
