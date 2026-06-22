/// LeetCode #2039 - The Time When the Network Becomes Idle
use std::collections::VecDeque;

fn network_becomes_idle(edges: Vec<Vec<i32>>, patience: Vec<i32>) -> i32 {
    let n = patience.len();
    let mut g = vec![Vec::new(); n];
    for e in edges {
        let u = e[0] as usize;
        let v = e[1] as usize;
        g[u].push(v);
        g[v].push(u);
    }

    let mut q = VecDeque::new();
    q.push_back(0);
    let mut vis = vec![false; n];
    vis[0] = true;
    let mut ans = 0i32;
    let mut d = 0i32;
    while !q.is_empty() {
        d += 1;
        let t = d * 2;
        for _ in 0..q.len() {
            let u = q.pop_front().unwrap();
            for &v in &g[u] {
                if !vis[v] {
                    vis[v] = true;
                    q.push_back(v);
                    let p = patience[v];
                    ans = ans.max((t - 1) / p * p + t + 1);
                }
            }
        }
    }
    ans
}

fn main() {
    println!(
        "{}",
        network_becomes_idle(vec![vec![0, 1], vec![1, 2]], vec![0, 2, 1])
    );
}

#[cfg(test)]
mod tests {
    use super::network_becomes_idle;

    #[test]
    fn example_one() {
        assert_eq!(
            network_becomes_idle(vec![vec![0, 1], vec![1, 2]], vec![0, 2, 1]),
            8
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(
            network_becomes_idle(
                vec![vec![0, 1], vec![0, 2], vec![1, 2]],
                vec![0, 10, 10],
            ),
            3
        );
    }
}
