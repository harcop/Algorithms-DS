/// LeetCode #2359 - Find Closest Node to Given Two Nodes
use std::collections::VecDeque;

fn closest_meeting_node(edges: Vec<i32>, node1: i32, node2: i32) -> i32 {
    let n = edges.len();
    let mut g = vec![Vec::new(); n];
    for i in 0..n {
        if edges[i] != -1 {
            g[i].push(edges[i] as usize);
        }
    }
    const INF: i32 = 1 << 30;
    let f = |start: usize| -> Vec<i32> {
        let mut dist = vec![INF; n];
        dist[start] = 0;
        let mut q = VecDeque::new();
        q.push_back(start);
        while let Some(i) = q.pop_front() {
            for &j in &g[i] {
                if dist[j] == INF {
                    dist[j] = dist[i] + 1;
                    q.push_back(j);
                }
            }
        }
        dist
    };
    let d1 = f(node1 as usize);
    let d2 = f(node2 as usize);
    let mut ans = -1;
    let mut best = INF;
    for i in 0..n {
        let t = d1[i].max(d2[i]);
        if t < best {
            best = t;
            ans = i as i32;
        }
    }
    ans
}

fn main() {
    println!("{}", closest_meeting_node(vec![2, 2, 3, -1], 0, 1));
}

#[cfg(test)]
mod tests {
    use super::closest_meeting_node;

    #[test]
    fn example_one() {
        assert_eq!(closest_meeting_node(vec![2, 2, 3, -1], 0, 1), 2);
    }

    #[test]
    fn example_two() {
        assert_eq!(closest_meeting_node(vec![1, 2, -1], 0, 2), 2);
    }
}
