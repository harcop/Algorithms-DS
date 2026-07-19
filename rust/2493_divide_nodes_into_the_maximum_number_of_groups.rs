/// LeetCode #2493 - Divide Nodes Into the Maximum Number of Groups
use std::collections::VecDeque;

fn magnificent_sets(n: i32, edges: Vec<Vec<i32>>) -> i32 {
    let n = n as usize;
    let mut g = vec![Vec::new(); n];
    for e in edges {
        let a = (e[0] - 1) as usize;
        let b = (e[1] - 1) as usize;
        g[a].push(b);
        g[b].push(a);
    }

    let mut d = vec![0; n];
    for i in 0..n {
        let mut q = VecDeque::from([i]);
        let mut dist = vec![0; n];
        dist[i] = 1;
        let mut mx = 1;
        let mut root = i;
        while let Some(a) = q.pop_front() {
            root = root.min(a);
            for &b in &g[a] {
                if dist[b] == 0 {
                    dist[b] = dist[a] + 1;
                    mx = mx.max(dist[b]);
                    q.push_back(b);
                } else if (dist[b] as i32 - dist[a] as i32).abs() != 1 {
                    return -1;
                }
            }
        }
        d[root] = d[root].max(mx);
    }
    d.into_iter().sum()
}

fn main() {
    println!(
        "{}",
        magnificent_sets(
            6,
            vec![
                vec![1, 2],
                vec![1, 4],
                vec![1, 5],
                vec![2, 6],
                vec![2, 3],
                vec![4, 6]
            ]
        )
    );
}

#[cfg(test)]
mod tests {
    use super::magnificent_sets;

    #[test]
    fn example_one() {
        assert_eq!(
            magnificent_sets(
                6,
                vec![
                    vec![1, 2],
                    vec![1, 4],
                    vec![1, 5],
                    vec![2, 6],
                    vec![2, 3],
                    vec![4, 6]
                ]
            ),
            4
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(
            magnificent_sets(3, vec![vec![1, 2], vec![2, 3], vec![3, 1]]),
            -1
        );
    }
}
