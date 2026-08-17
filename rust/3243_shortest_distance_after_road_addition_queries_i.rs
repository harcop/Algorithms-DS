/// LeetCode #3243 - Shortest Distance After Road Addition Queries I
use std::collections::VecDeque;

fn shortest_distance_after_queries(n: i32, queries: Vec<Vec<i32>>) -> Vec<i32> {
    let n = n as usize;
    let mut g: Vec<Vec<usize>> = (0..n - 1).map(|i| vec![i + 1]).collect();
    g.push(vec![]);

    let bfs = |g: &[Vec<usize>]| -> i32 {
        let mut q = VecDeque::new();
        let mut vis = vec![false; n];
        q.push_back(0usize);
        vis[0] = true;
        let mut d = 0;
        loop {
            for _ in 0..q.len() {
                let u = q.pop_front().unwrap();
                if u == n - 1 {
                    return d;
                }
                for &v in &g[u] {
                    if !vis[v] {
                        vis[v] = true;
                        q.push_back(v);
                    }
                }
            }
            d += 1;
        }
    };

    let mut ans = Vec::new();
    for q in queries {
        let u = q[0] as usize;
        let v = q[1] as usize;
        g[u].push(v);
        ans.push(bfs(&g));
    }
    ans
}

fn main() {
    println!(
        "{:?}",
        shortest_distance_after_queries(5, vec![vec![2, 4], vec![0, 2], vec![0, 4]])
    );
}

#[cfg(test)]
mod tests {
    use super::shortest_distance_after_queries;

    #[test]
    fn example1() {
        assert_eq!(
            shortest_distance_after_queries(5, vec![vec![2, 4], vec![0, 2], vec![0, 4]]),
            vec![3, 2, 1]
        );
    }

    #[test]
    fn example2() {
        assert_eq!(
            shortest_distance_after_queries(4, vec![vec![0, 3], vec![0, 2]]),
            vec![1, 1]
        );
    }
}
