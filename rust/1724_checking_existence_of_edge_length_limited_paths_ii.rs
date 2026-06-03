/// LeetCode #1724 - Checking Existence of Edge Length Limited Paths II
use std::collections::BTreeMap;

struct Puf {
    parent: Vec<BTreeMap<i32, i32>>,
}

impl Puf {
    fn new(n: i32) -> Self {
        let n = n as usize;
        let mut parent = Vec::with_capacity(n);
        for i in 0..n {
            let mut m = BTreeMap::new();
            m.insert(0, i as i32);
            parent.push(m);
        }
        Self { parent }
    }

    fn find(&mut self, u: i32, limit: i32) -> i32 {
        let uu = u as usize;
        let p = *self.parent[uu].range(..=limit).next_back().unwrap().1;
        if p != u {
            let root = self.find(p, limit);
            self.parent[uu].insert(limit, root);
            root
        } else {
            u
        }
    }

    fn unite(&mut self, u: i32, v: i32, limit: i32) {
        let i = self.find(u, limit);
        let j = self.find(v, limit);
        if i != j {
            self.parent[i as usize].insert(limit, j);
        }
    }
}

pub struct DistanceLimitedPathsExist {
    puf: Puf,
}

impl DistanceLimitedPathsExist {
    fn new(n: i32, mut edge_list: Vec<Vec<i32>>) -> Self {
        let mut puf = Puf::new(n);
        edge_list.sort_unstable_by_key(|e| e[2]);
        for e in edge_list {
            puf.unite(e[0], e[1], e[2]);
        }
        Self { puf }
    }

    fn query(&mut self, p: i32, q: i32, limit: i32) -> bool {
        self.puf.find(p, limit - 1) == self.puf.find(q, limit - 1)
    }
}

fn run_queries(n: i32, edge_list: Vec<Vec<i32>>, queries: Vec<Vec<i32>>) -> Vec<bool> {
    let mut d = DistanceLimitedPathsExist::new(n, edge_list);
    queries
        .iter()
        .map(|q| d.query(q[0], q[1], q[2]))
        .collect()
}

fn main() {
    println!(
        "{:?}",
        run_queries(
            6,
            vec![
                vec![0, 2, 4],
                vec![0, 3, 2],
                vec![1, 2, 3],
                vec![2, 3, 1],
                vec![4, 5, 5],
            ],
            vec![vec![2, 3, 2], vec![1, 3, 3], vec![2, 0, 3], vec![0, 5, 6]],
        )
    );
}
#[cfg(test)]
mod tests {
    use super::run_queries;
    #[test]
    fn example_one() {
        assert_eq!(
            run_queries(
                6,
                vec![
                    vec![0, 2, 4],
                    vec![0, 3, 2],
                    vec![1, 2, 3],
                    vec![2, 3, 1],
                    vec![4, 5, 5],
                ],
                vec![vec![2, 3, 2], vec![1, 3, 3], vec![2, 0, 3], vec![0, 5, 6]],
            ),
            vec![true, false, true, false]
        );
    }
}
