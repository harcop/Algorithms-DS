/// LeetCode #3543 - Maximum Weighted K-Edge Path
fn max_weight(n: i32, edges: Vec<Vec<i32>>, k: i32, t: i32) -> i32 {
    let n = n as usize;
    let k = k as usize;
    let mut adj = vec![Vec::new(); n];
    for e in &edges {
        adj[e[0] as usize].push((e[1] as usize, e[2]));
    }
    let mut dp: Vec<std::collections::HashSet<i32>> = vec![std::collections::HashSet::new(); n];
    for s in dp.iter_mut() {
        s.insert(0);
    }
    for _ in 0..k {
        let mut nxt: Vec<std::collections::HashSet<i32>> = vec![std::collections::HashSet::new(); n];
        for i in 0..n {
            for &c in &dp[i] {
                for &(j, w) in &adj[i] {
                    if c + w < t {
                        nxt[j].insert(c + w);
                    }
                }
            }
        }
        dp = nxt;
    }
    let mut ans = -1;
    for s in &dp {
        for &x in s {
            ans = ans.max(x);
        }
    }
    ans
}

fn main() {
    println!("{}", max_weight(3, vec![vec![0, 1, 1], vec![1, 2, 2]], 2, 4));
}

#[cfg(test)]
mod tests {
    use super::max_weight;

    #[test]
    fn example1() {
        assert_eq!(max_weight(3, vec![vec![0, 1, 1], vec![1, 2, 2]], 2, 4), 3);
    }

    #[test]
    fn example2() {
        assert_eq!(max_weight(3, vec![vec![0, 1, 2], vec![0, 2, 3]], 1, 3), 2);
    }

    #[test]
    fn example3() {
        assert_eq!(max_weight(3, vec![vec![0, 1, 6], vec![1, 2, 8]], 1, 6), -1);
    }
}
