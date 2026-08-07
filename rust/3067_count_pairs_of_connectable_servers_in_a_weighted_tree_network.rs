/// LeetCode #3067 - Count Pairs of Connectable Servers in a Weighted Tree Network
fn count_pairs_of_connectable_servers(edges: Vec<Vec<i32>>, signal_speed: i32) -> Vec<i64> {
    let n = edges.len() + 1;
    let mut adj = vec![vec![]; n];
    for e in edges {
        let a = e[0] as usize;
        let b = e[1] as usize;
        let w = e[2];
        adj[a].push((b, w));
        adj[b].push((a, w));
    }

    let mut ans = vec![0i64; n];

    fn dfs(
        adj: &[Vec<(usize, i32)>],
        u: usize,
        parent: usize,
        dist: i32,
        signal_speed: i32,
    ) -> i64 {
        let mut cnt = if dist % signal_speed == 0 { 1 } else { 0 };
        for &(v, w) in &adj[u] {
            if v == parent {
                continue;
            }
            cnt += dfs(adj, v, u, dist + w, signal_speed);
        }
        cnt
    }

    for center in 0..n {
        let mut seen = 0i64;
        for &(nb, w) in &adj[center] {
            let t = dfs(&adj, nb, center, w, signal_speed);
            ans[center] += seen * t;
            seen += t;
        }
    }

    ans
}

fn main() {
    let edges = vec![
        vec![0, 1, 1],
        vec![1, 2, 5],
        vec![2, 3, 13],
        vec![3, 4, 9],
        vec![4, 5, 2],
    ];
    println!("{:?}", count_pairs_of_connectable_servers(edges, 1));
}

#[cfg(test)]
mod tests {
    use super::count_pairs_of_connectable_servers;

    #[test]
    fn example1() {
        let edges = vec![
            vec![0, 1, 1],
            vec![1, 2, 5],
            vec![2, 3, 13],
            vec![3, 4, 9],
            vec![4, 5, 2],
        ];
        assert_eq!(
            count_pairs_of_connectable_servers(edges, 1),
            vec![0, 4, 6, 6, 4, 0]
        );
    }

    #[test]
    fn example2() {
        let edges = vec![
            vec![0, 6, 3],
            vec![6, 5, 3],
            vec![0, 3, 1],
            vec![3, 2, 7],
            vec![3, 1, 6],
            vec![3, 4, 2],
        ];
        assert_eq!(
            count_pairs_of_connectable_servers(edges, 3),
            vec![2, 0, 0, 0, 0, 0, 2]
        );
    }
}
