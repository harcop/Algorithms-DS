/// LeetCode #2045 - Second Minimum Time to Reach Destination
use std::collections::VecDeque;

const INF: i32 = i32::MAX / 2;

fn second_minimum(n: i32, edges: Vec<Vec<i32>>, time: i32, change: i32) -> i32 {
    let n = n as usize;
    let mut g = vec![Vec::new(); n + 1];
    for e in edges {
        let u = e[0] as usize;
        let v = e[1] as usize;
        g[u].push(v);
        g[v].push(u);
    }

    let mut dist = vec![[INF; 2]; n + 1];
    dist[1][0] = 0;
    let mut q = VecDeque::new();
    q.push_back((1usize, 0i32));

    while let Some((u, d)) = q.pop_front() {
        for &v in &g[u] {
            if d + 1 < dist[v][0] {
                dist[v][0] = d + 1;
                q.push_back((v, d + 1));
            } else if dist[v][0] < d + 1 && d + 1 < dist[v][1] {
                dist[v][1] = d + 1;
                if v == n {
                    break;
                }
                q.push_back((v, d + 1));
            }
        }
    }

    let steps = dist[n][1];
    let mut ans = 0i32;
    for i in 0..steps {
        ans += time;
        if i < steps - 1 && (ans / change) % 2 == 1 {
            ans = (ans + change) / change * change;
        }
    }
    ans
}

fn main() {
    println!(
        "{}",
        second_minimum(
            5,
            vec![vec![1, 2], vec![1, 3], vec![1, 4], vec![3, 4], vec![4, 5]],
            3,
            5,
        )
    );
}

#[cfg(test)]
mod tests {
    use super::second_minimum;

    #[test]
    fn example_one() {
        assert_eq!(
            second_minimum(
                5,
                vec![vec![1, 2], vec![1, 3], vec![1, 4], vec![3, 4], vec![4, 5]],
                3,
                5,
            ),
            13
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(second_minimum(2, vec![vec![1, 2]], 3, 2), 11);
    }
}
