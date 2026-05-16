/// LeetCode #834 - Sum of Distances in Tree
fn sum_of_distances_in_tree(n: i32, edges: Vec<Vec<i32>>) -> Vec<i32> {
    let n = n as usize;
    let mut g = vec![vec![]; n];
    for e in edges {
        let u = e[0] as usize;
        let v = e[1] as usize;
        g[u].push(v);
        g[v].push(u);
    }
    let mut count = vec![0i32; n];
    let mut sub = vec![0i64; n];

    fn dfs1(g: &[Vec<usize>], u: usize, p: usize, count: &mut [i32], sub: &mut [i64]) {
        count[u] = 1;
        sub[u] = 0;
        for &v in &g[u] {
            if v == p {
                continue;
            }
            dfs1(g, v, u, count, sub);
            count[u] += count[v];
            sub[u] += sub[v] + count[v] as i64;
        }
    }
    dfs1(&g, 0, n, &mut count, &mut sub);

    let mut res = vec![0i32; n];
    res[0] = sub[0] as i32;

    fn dfs2(g: &[Vec<usize>], u: usize, p: usize, count: &[i32], ans: &mut [i32]) {
        for &v in &g[u] {
            if v == p {
                continue;
            }
            ans[v] = ans[u] - count[v] + (count[0] - count[v]);
            dfs2(g, v, u, count, ans);
        }
    }
    dfs2(&g, 0, n, &count, &mut res);
    res
}

fn main() {
    println!(
        "{:?}",
        sum_of_distances_in_tree(6, vec![vec![0, 1], vec![0, 2], vec![2, 3], vec![2, 4], vec![2, 5]])
    );
}

#[cfg(test)]
mod tests {
    use super::sum_of_distances_in_tree;

    #[test]
    fn example_one() {
        assert_eq!(
            sum_of_distances_in_tree(
                6,
                vec![vec![0, 1], vec![0, 2], vec![2, 3], vec![2, 4], vec![2, 5]],
            ),
            vec![8, 12, 6, 10, 10, 10]
        );
    }
}
