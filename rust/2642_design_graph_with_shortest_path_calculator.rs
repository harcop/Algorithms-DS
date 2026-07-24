/// LeetCode #2642 - Design Graph With Shortest Path Calculator
struct Graph {
    g: Vec<Vec<i32>>,
}

impl Graph {
    const INF: i32 = 1 << 29;

    fn new(n: i32, edges: Vec<Vec<i32>>) -> Self {
        let n = n as usize;
        let mut g = vec![vec![Self::INF; n]; n];
        for e in edges {
            g[e[0] as usize][e[1] as usize] = e[2];
        }
        Graph { g }
    }

    fn add_edge(&mut self, edge: Vec<i32>) {
        self.g[edge[0] as usize][edge[1] as usize] = edge[2];
    }

    fn shortest_path(&self, node1: i32, node2: i32) -> i32 {
        let n = self.g.len();
        let mut dist = vec![Self::INF; n];
        let mut vis = vec![false; n];
        dist[node1 as usize] = 0;
        for _ in 0..n {
            let mut t = None;
            for j in 0..n {
                if !vis[j] && (t.is_none() || dist[j] < dist[t.unwrap()]) {
                    t = Some(j);
                }
            }
            let t = t.unwrap();
            vis[t] = true;
            for j in 0..n {
                dist[j] = dist[j].min(dist[t] + self.g[t][j]);
            }
        }
        if dist[node2 as usize] >= Self::INF {
            -1
        } else {
            dist[node2 as usize]
        }
    }
}

fn main() {
    let g = Graph::new(
        4,
        vec![
            vec![0, 2, 5],
            vec![0, 1, 2],
            vec![1, 2, 1],
            vec![3, 0, 3],
        ],
    );
    println!("{}", g.shortest_path(3, 2));
}

#[cfg(test)]
mod tests {
    use super::Graph;

    #[test]
    fn example_one() {
        let mut g = Graph::new(
            4,
            vec![
                vec![0, 2, 5],
                vec![0, 1, 2],
                vec![1, 2, 1],
                vec![3, 0, 3],
            ],
        );
        assert_eq!(g.shortest_path(3, 2), 6);
        assert_eq!(g.shortest_path(0, 3), -1);
        g.add_edge(vec![1, 3, 4]);
        assert_eq!(g.shortest_path(0, 3), 6);
    }
}
