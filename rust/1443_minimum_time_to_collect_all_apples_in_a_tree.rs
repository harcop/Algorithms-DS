/// LeetCode #1443 - Minimum Time To Collect All Apples In A Tree
fn min_time(n: i32, edges: Vec<Vec<i32>>, has_apple: Vec<bool>) -> i32 {
    let n = n as usize;
    let mut adj = vec![vec![]; n];
    for e in edges {
        let (u, v) = (e[0] as usize, e[1] as usize);
        adj[u].push(v);
        adj[v].push(u);
    }
    fn dfs(u: usize, p: usize, adj: &[Vec<usize>], has_apple: &[bool]) -> i32 {
        let mut steps = 0;
        for &v in &adj[u] {
            if v == p { continue; }
            let sub = dfs(v, u, adj, has_apple);
            if sub > 0 || has_apple[v] { steps += sub + 2; }
        }
        steps
    }
    dfs(0, n, &adj, &has_apple)
}
fn main() {
    println!("{}", min_time(7, vec![vec![0,1],vec![0,2],vec![1,4],vec![1,5],vec![2,3],vec![2,6]], vec![false,false,true,false,true,true,false]));
}
#[cfg(test)]
mod tests {
    use super::min_time;
    #[test]
    fn example_one() {
        assert_eq!(min_time(7, vec![vec![0,1],vec![0,2],vec![1,4],vec![1,5],vec![2,3],vec![2,6]], vec![false,false,true,false,true,true,false]), 8);
    }
    #[test]
    fn example_two() {
        assert_eq!(min_time(7, vec![vec![0,1],vec![0,2],vec![1,4],vec![1,5],vec![2,3],vec![2,6]], vec![false,false,false,false,false,false,false]), 0);
    }
}