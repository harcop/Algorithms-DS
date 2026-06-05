/// LeetCode #1761 - Minimum Degree of a Connected Trio in a Graph
fn min_trio_degree(n: i32, edges: Vec<Vec<i32>>) -> i32 {
    let n = n as usize;
    let mut adj = vec![vec![]; n];
    for e in &edges {
        let u = (e[0] - 1) as usize;
        let v = (e[1] - 1) as usize;
        adj[u].push(v);
        adj[v].push(u);
    }
    for a in &mut adj {
        a.sort_unstable();
    }
    let mut ans = i32::MAX;
    for u in 0..n {
        for &v in &adj[u] {
            if v <= u {
                continue;
            }
            for &w in &adj[u] {
                if w == v {
                    continue;
                }
                if adj[v].binary_search(&w).is_ok() {
                    let mut cnt = 0i32;
                    for e in &edges {
                        let a = (e[0] - 1) as usize;
                        let b = (e[1] - 1) as usize;
                        if (a == u || a == v || a == w) && (b == u || b == v || b == w) {
                            cnt += 1;
                        }
                    }
                    ans = ans.min(cnt);
                }
            }
        }
    }
    if ans == i32::MAX { -1 } else { ans }
}
fn main() {
    println!(
        "{}",
        min_trio_degree(
            6,
            vec![vec![1, 2], vec![1, 3], vec![3, 2], vec![4, 5], vec![3, 4], vec![5, 6]],
        )
    );
}
#[cfg(test)]
mod tests {
    use super::min_trio_degree;
    #[test]
    fn example_one() {
        assert_eq!(
            min_trio_degree(
                6,
                vec![vec![1, 2], vec![1, 3], vec![3, 2], vec![4, 5], vec![3, 4], vec![5, 6]],
            ),
            3
        );
    }
    #[test]
    fn example_two() {
        assert_eq!(min_trio_degree(7, vec![vec![1, 2], vec![3, 4], vec![5, 6]]), -1);
    }
}
