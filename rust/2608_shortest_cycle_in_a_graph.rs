/// LeetCode #2608 - Shortest Cycle in a Graph
use std::collections::VecDeque;

fn find_shortest_cycle(n: i32, edges: Vec<Vec<i32>>) -> i32 {
    let n = n as usize;
    let mut g = vec![Vec::new(); n];
    for e in &edges {
        let u = e[0] as usize;
        let v = e[1] as usize;
        g[u].push(v);
        g[v].push(u);
    }

    const INF: i32 = 1 << 30;
    let bfs = |start: usize| -> i32 {
        let mut dist = vec![-1; n];
        dist[start] = 0;
        let mut q = VecDeque::new();
        q.push_back((start, usize::MAX));
        let mut ans = INF;
        while let Some((u, fa)) = q.pop_front() {
            for &v in &g[u] {
                if dist[v] < 0 {
                    dist[v] = dist[u] + 1;
                    q.push_back((v, u));
                } else if v != fa {
                    ans = ans.min(dist[u] + dist[v] + 1);
                }
            }
        }
        ans
    };

    let mut ans = INF;
    for i in 0..n {
        ans = ans.min(bfs(i));
    }
    if ans < INF {
        ans
    } else {
        -1
    }
}

fn main() {
    println!(
        "{}",
        find_shortest_cycle(
            7,
            vec![
                vec![0, 1],
                vec![1, 2],
                vec![2, 0],
                vec![3, 4],
                vec![4, 5],
                vec![5, 6],
                vec![6, 3]
            ]
        )
    );
}

#[cfg(test)]
mod tests {
    use super::find_shortest_cycle;

    #[test]
    fn example_one() {
        assert_eq!(
            find_shortest_cycle(
                7,
                vec![
                    vec![0, 1],
                    vec![1, 2],
                    vec![2, 0],
                    vec![3, 4],
                    vec![4, 5],
                    vec![5, 6],
                    vec![6, 3]
                ]
            ),
            3
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(find_shortest_cycle(4, vec![vec![0, 1], vec![0, 2]]), -1);
    }
}
