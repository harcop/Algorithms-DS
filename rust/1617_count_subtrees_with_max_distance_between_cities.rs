/// LeetCode #1617 - Count Subtrees With Max Distance Between Cities
fn count_subgraphs_for_each_diameter(n: i32, edges: Vec<Vec<i32>>) -> Vec<i32> {
    let n = n as usize;
    let off = if edges.iter().any(|e| e[0] == 0 || e[1] == 0) { 0 } else { 1 };
    let mut adj = vec![vec![]; n];
    for e in &edges {
        let (u, v) = (e[0] as usize - off, e[1] as usize - off);
        adj[u].push(v);
        adj[v].push(u);
    }
    let mut ans = vec![0i32; n.saturating_sub(1)];
    for mask in 1..(1usize << n) {
        let mut nodes = vec![];
        for i in 0..n {
            if mask & (1 << i) != 0 {
                nodes.push(i);
            }
        }
        if nodes.len() < 2 {
            continue;
        }
        let mut seen = vec![false; n];
        let start = nodes[0];
        let mut stack = vec![start];
        seen[start] = true;
        let mut cnt = 0usize;
        while let Some(u) = stack.pop() {
            cnt += 1;
            for &v in &adj[u] {
                if !seen[v] && (mask & (1 << v)) != 0 {
                    seen[v] = true;
                    stack.push(v);
                }
            }
        }
        if cnt != nodes.len() {
            continue;
        }
        let mut diam = 0i32;
        for &src in &nodes {
            let mut dist = vec![-1i32; n];
            dist[src] = 0;
            let mut q = vec![src];
            let mut qi = 0usize;
            while qi < q.len() {
                let u = q[qi];
                qi += 1;
                for &v in &adj[u] {
                    if dist[v] == -1 && (mask & (1 << v)) != 0 {
                        dist[v] = dist[u] + 1;
                        q.push(v);
                    }
                }
            }
            for &v in &nodes {
                diam = diam.max(dist[v]);
            }
        }
        if diam > 0 {
            ans[diam as usize - 1] += 1;
        }
    }
    ans
}
fn main() {
    println!("{:?}", count_subgraphs_for_each_diameter(4, vec![vec![0, 1], vec![1, 2], vec![2, 3]]));
}
#[cfg(test)]
mod tests {
    use super::count_subgraphs_for_each_diameter;
    #[test]
    fn example_one() {
        assert_eq!(
            count_subgraphs_for_each_diameter(4, vec![vec![1, 2], vec![2, 3], vec![2, 4]]),
            vec![3, 4, 0]
        );
    }
    #[test]
    fn path_graph() {
        assert_eq!(
            count_subgraphs_for_each_diameter(4, vec![vec![0, 1], vec![1, 2], vec![2, 3]]),
            vec![3, 2, 1]
        );
    }
}
