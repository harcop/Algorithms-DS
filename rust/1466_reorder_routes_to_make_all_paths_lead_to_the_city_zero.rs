/// LeetCode #1466 - Reorder Routes To Make All Paths Lead To The City Zero
fn min_reorder(n: i32, connections: Vec<Vec<i32>>) -> i32 {
    let n = n as usize;
    let mut adj = vec![vec![]; n];
    for c in connections {
        let (a, b) = (c[0] as usize, c[1] as usize);
        adj[a].push((b, 1));
        adj[b].push((a, 0));
    }
    fn dfs(u: usize, p: usize, adj: &[Vec<(usize, i32)>]) -> i32 {
        let mut ch = 0;
        for &(v, d) in &adj[u] {
            if v == p { continue; }
            ch += dfs(v, u, adj) + d;
        }
        ch
    }
    dfs(0, n, &adj)
}
fn main() { println!("{}", min_reorder(6, vec![vec![0,1],vec![1,3],vec![2,3],vec![4,0],vec![4,5]])); }
#[cfg(test)]
mod tests {
    use super::min_reorder;
    #[test]
    fn example_one() { assert_eq!(min_reorder(6, vec![vec![0,1],vec![1,3],vec![2,3],vec![4,0],vec![4,5]]), 3); }
    #[test]
    fn example_two() { assert_eq!(min_reorder(5, vec![vec![1,0],vec![1,2],vec![3,2],vec![3,4]]), 2); }
}