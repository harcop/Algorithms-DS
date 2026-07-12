/// LeetCode #2368 - Reachable Nodes With Restrictions
fn reachable_nodes(n: i32, edges: Vec<Vec<i32>>, restricted: Vec<i32>) -> i32 {
    let n = n as usize;
    let mut tree = vec![vec![]; n];
    let mut seen = vec![false; n];

    for edge in &edges {
        let u = edge[0] as usize;
        let v = edge[1] as usize;
        tree[u].push(v);
        tree[v].push(u);
    }

    for r in restricted {
        seen[r as usize] = true;
    }

    dfs(&tree, 0, &mut seen)
}

fn dfs(tree: &[Vec<usize>], u: usize, seen: &mut [bool]) -> i32 {
    if seen[u] {
        return 0;
    }
    seen[u] = true;
    let mut ans = 1;
    for &v in &tree[u] {
        ans += dfs(tree, v, seen);
    }
    ans
}

fn main() {
    println!(
        "{}",
        reachable_nodes(
            7,
            vec![
                vec![0, 1],
                vec![1, 2],
                vec![3, 1],
                vec![4, 0],
                vec![0, 5],
                vec![5, 6]
            ],
            vec![4, 5]
        )
    );
}

#[cfg(test)]
mod tests {
    use super::reachable_nodes;

    #[test]
    fn example_one() {
        assert_eq!(
            reachable_nodes(
                7,
                vec![
                    vec![0, 1],
                    vec![1, 2],
                    vec![3, 1],
                    vec![4, 0],
                    vec![0, 5],
                    vec![5, 6]
                ],
                vec![4, 5]
            ),
            4
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(
            reachable_nodes(
                7,
                vec![
                    vec![0, 1],
                    vec![0, 2],
                    vec![0, 5],
                    vec![0, 4],
                    vec![3, 2],
                    vec![6, 5]
                ],
                vec![4, 2, 1]
            ),
            3
        );
    }
}
