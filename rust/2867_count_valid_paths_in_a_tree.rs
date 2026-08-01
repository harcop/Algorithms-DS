/// LeetCode #2867 - Count Valid Paths in a Tree
struct UnionFind {
    parent: Vec<usize>,
    size: Vec<i64>,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        Self {
            parent: (0..=n).collect(),
            size: vec![1; n + 1],
        }
    }

    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x]);
        }
        self.parent[x]
    }

    fn union(&mut self, a: usize, b: usize) {
        let mut root_a = self.find(a);
        let mut root_b = self.find(b);
        if root_a == root_b {
            return;
        }
        if self.size[root_a] < self.size[root_b] {
            std::mem::swap(&mut root_a, &mut root_b);
        }
        self.parent[root_b] = root_a;
        self.size[root_a] += self.size[root_b];
    }

    fn component_size(&mut self, x: usize) -> i64 {
        let root = self.find(x);
        self.size[root]
    }
}

fn count_paths(n: i32, edges: Vec<Vec<i32>>) -> i64 {
    let n = n as usize;
    let mut is_prime = vec![true; n + 1];
    is_prime[0] = false;
    if n >= 1 {
        is_prime[1] = false;
    }
    let mut p = 2;
    while p * p <= n {
        if is_prime[p] {
            for multiple in (p * p..=n).step_by(p) {
                is_prime[multiple] = false;
            }
        }
        p += 1;
    }

    let mut graph = vec![Vec::new(); n + 1];
    let mut union_find = UnionFind::new(n);
    for edge in edges {
        let a = edge[0] as usize;
        let b = edge[1] as usize;
        graph[a].push(b);
        graph[b].push(a);
        if !is_prime[a] && !is_prime[b] {
            union_find.union(a, b);
        }
    }

    let mut answer = 0_i64;
    for node in 1..=n {
        if !is_prime[node] {
            continue;
        }
        let mut previous_nodes = 0_i64;
        for &neighbor in &graph[node] {
            if is_prime[neighbor] {
                continue;
            }
            let component_nodes = union_find.component_size(neighbor);
            answer += component_nodes * (previous_nodes + 1);
            previous_nodes += component_nodes;
        }
    }
    answer
}

fn main() {
    println!(
        "{}",
        count_paths(5, vec![vec![1, 2], vec![1, 3], vec![2, 4], vec![2, 5]])
    );
}

#[cfg(test)]
mod tests {
    use super::count_paths;

    #[test]
    fn example_one() {
        assert_eq!(
            count_paths(5, vec![vec![1, 2], vec![1, 3], vec![2, 4], vec![2, 5]]),
            4
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(
            count_paths(
                6,
                vec![vec![1, 2], vec![1, 3], vec![2, 4], vec![3, 5], vec![3, 6]]
            ),
            6
        );
    }
}
