/// LeetCode #3615 - Longest Palindromic Path in Graph
fn max_len(n: i32, edges: Vec<Vec<i32>>, label: String) -> i32 {
    let n = n as usize;
    let label = label.as_bytes();
    let mut adj = vec![Vec::new(); n];
    let mut dp = vec![vec![vec![false; n]; n]; 1 << n];
    for e in &edges {
        let (u, v) = (e[0] as usize, e[1] as usize);
        adj[u].push(v);
        adj[v].push(u);
        if label[u] == label[v] {
            let (a, b) = if u < v { (u, v) } else { (v, u) };
            dp[(1 << u) | (1 << v)][a][b] = true;
        }
    }
    for i in 0..n {
        dp[1 << i][i][i] = true;
    }
    let mut ans = 0;
    for mask in 1..(1 << n) {
        for u in 0..n {
            for v in u..n {
                if !dp[mask][u][v] {
                    continue;
                }
                ans = ans.max(mask.count_ones() as i32);
                for &nu in &adj[u] {
                    if mask & (1 << nu) != 0 {
                        continue;
                    }
                    for &nv in &adj[v] {
                        if mask & (1 << nv) != 0 || nu == nv {
                            continue;
                        }
                        if label[nu] == label[nv] {
                            let nmask = mask | (1 << nu) | (1 << nv);
                            let (a, b) = if nu < nv { (nu, nv) } else { (nv, nu) };
                            dp[nmask][a][b] = true;
                        }
                    }
                }
            }
        }
    }
    ans
}

fn main() {
    println!("{}", max_len(3, vec![vec![0, 1], vec![1, 2]], "aba".into()));
}

#[cfg(test)]
mod tests {
    use super::max_len;

    #[test]
    fn example1() {
        assert_eq!(max_len(3, vec![vec![0, 1], vec![1, 2]], "aba".into()), 3);
    }

    #[test]
    fn example2() {
        assert_eq!(max_len(3, vec![vec![0, 1], vec![0, 2]], "abc".into()), 1);
    }

    #[test]
    fn example3() {
        assert_eq!(
            max_len(4, vec![vec![0, 2], vec![0, 3], vec![3, 1]], "bbac".into()),
            3
        );
    }
}
