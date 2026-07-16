/// LeetCode #2421 - Number of Good Paths
use std::collections::HashMap;

struct Dsu {
    parent: Vec<usize>,
    size: Vec<usize>,
}

impl Dsu {
    fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(),
            size: vec![1; n],
        }
    }

    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x]);
        }
        self.parent[x]
    }

    fn union(&mut self, a: usize, b: usize) {
        let mut pa = self.find(a);
        let mut pb = self.find(b);
        if pa == pb {
            return;
        }
        if self.size[pa] < self.size[pb] {
            std::mem::swap(&mut pa, &mut pb);
        }
        self.parent[pb] = pa;
        self.size[pa] += self.size[pb];
    }
}

fn number_of_good_paths(vals: Vec<i32>, edges: Vec<Vec<i32>>) -> i32 {
    let n = vals.len();
    let mut graph = vec![Vec::new(); n];
    for edge in edges {
        let a = edge[0] as usize;
        let b = edge[1] as usize;
        graph[a].push(b);
        graph[b].push(a);
    }

    let mut order: Vec<usize> = (0..n).collect();
    order.sort_unstable_by_key(|&i| vals[i]);
    let mut dsu = Dsu::new(n);
    let mut active = vec![false; n];
    let mut ans = 0;
    let mut i = 0;

    while i < n {
        let value = vals[order[i]];
        let mut j = i;
        while j < n && vals[order[j]] == value {
            let node = order[j];
            active[node] = true;
            for &next in &graph[node] {
                if active[next] {
                    dsu.union(node, next);
                }
            }
            j += 1;
        }

        let mut count: HashMap<usize, i32> = HashMap::new();
        for &node in &order[i..j] {
            *count.entry(dsu.find(node)).or_insert(0) += 1;
        }
        for freq in count.into_values() {
            ans += freq * (freq + 1) / 2;
        }
        i = j;
    }

    ans
}

fn main() {
    println!("{}", number_of_good_paths(vec![1, 3, 2, 1, 3], vec![vec![0, 1], vec![0, 2], vec![2, 3], vec![2, 4]]));
}

#[cfg(test)]
mod tests {
    use super::number_of_good_paths;

    #[test]
    fn example_one() {
        assert_eq!(
            number_of_good_paths(
                vec![1, 3, 2, 1, 3],
                vec![vec![0, 1], vec![0, 2], vec![2, 3], vec![2, 4]]
            ),
            6
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(
            number_of_good_paths(vec![1, 1, 2, 2, 3], vec![vec![0, 1], vec![1, 2], vec![2, 3], vec![2, 4]]),
            7
        );
    }
}
