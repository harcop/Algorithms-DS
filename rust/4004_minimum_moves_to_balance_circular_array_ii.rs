/// LeetCode #4004 - Minimum Moves to Balance Circular Array II
use std::collections::VecDeque;

struct Edge {
    to: usize,
    cap: i64,
    cost: i64,
    rev: usize,
}

fn add_edge(graph: &mut [Vec<Edge>], u: usize, v: usize, cap: i64, cost: i64) {
    let rev_u = graph[v].len();
    let rev_v = graph[u].len();
    graph[u].push(Edge {
        to: v,
        cap,
        cost,
        rev: rev_u,
    });
    graph[v].push(Edge {
        to: u,
        cap: 0,
        cost: -cost,
        rev: rev_v,
    });
}

fn min_moves(balance: Vec<i32>) -> i64 {
    let total: i64 = balance.iter().map(|&x| x as i64).sum();
    if total < 0 {
        return -1;
    }
    let n = balance.len();
    let total_deficit: i64 = balance
        .iter()
        .filter(|&&x| x < 0)
        .map(|&x| -(x as i64))
        .sum();
    if total_deficit == 0 {
        return 0;
    }

    let source = n;
    let sink = n + 1;
    let mut graph: Vec<Vec<Edge>> = (0..n + 2).map(|_| Vec::new()).collect();
    let cap_inf = 1_000_000_000i64;
    for i in 0..n {
        if balance[i] > 0 {
            add_edge(&mut graph, source, i, balance[i] as i64, 0);
        } else if balance[i] < 0 {
            add_edge(&mut graph, i, sink, -(balance[i] as i64), 0);
        }
        let nxt = (i + 1) % n;
        add_edge(&mut graph, i, nxt, cap_inf, 1);
        add_edge(&mut graph, nxt, i, cap_inf, 1);
    }

    let inf = i64::MAX / 4;
    let mut total_cost = 0i64;
    let mut current_flow = 0i64;
    while current_flow < total_deficit {
        let mut dist = vec![inf; n + 2];
        let mut parent_node = vec![0usize; n + 2];
        let mut parent_edge = vec![0usize; n + 2];
        let mut in_queue = vec![false; n + 2];
        let mut queue = VecDeque::new();
        queue.push_back(source);
        dist[source] = 0;
        in_queue[source] = true;
        while let Some(u) = queue.pop_front() {
            in_queue[u] = false;
            for (idx, e) in graph[u].iter().enumerate() {
                if e.cap > 0 && dist[e.to] > dist[u] + e.cost {
                    dist[e.to] = dist[u] + e.cost;
                    parent_node[e.to] = u;
                    parent_edge[e.to] = idx;
                    if !in_queue[e.to] {
                        in_queue[e.to] = true;
                        queue.push_back(e.to);
                    }
                }
            }
        }
        if dist[sink] >= inf {
            return -1;
        }
        let mut push = total_deficit - current_flow;
        let mut curr = sink;
        while curr != source {
            let p = parent_node[curr];
            let idx = parent_edge[curr];
            push = push.min(graph[p][idx].cap);
            curr = p;
        }
        curr = sink;
        while curr != source {
            let p = parent_node[curr];
            let idx = parent_edge[curr];
            let rev = graph[p][idx].rev;
            graph[p][idx].cap -= push;
            graph[curr][rev].cap += push;
            curr = p;
        }
        current_flow += push;
        total_cost += push * dist[sink];
    }
    total_cost
}

fn main() {
    println!("{}", min_moves(vec![-1, 2, -1]));
}

#[cfg(test)]
mod tests {
    use super::min_moves;

    #[test]
    fn example1() {
        assert_eq!(min_moves(vec![-1, 2, -1]), 2);
    }

    #[test]
    fn example2() {
        assert_eq!(min_moves(vec![4, -1, -2]), 3);
    }

    #[test]
    fn example3() {
        assert_eq!(min_moves(vec![-3, -3, 5]), -1);
    }
}
