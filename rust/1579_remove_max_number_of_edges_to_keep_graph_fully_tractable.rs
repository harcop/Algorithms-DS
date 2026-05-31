/// LeetCode #1579 - Remove Max Number Of Edges To Keep Graph Fully Traversable
struct Dsu {
    p: Vec<usize>,
    r: Vec<usize>,
}

impl Dsu {
    fn new(n: usize) -> Self {
        Dsu {
            p: (0..n).collect(),
            r: vec![0; n],
        }
    }
    fn find(&mut self, x: usize) -> usize {
        if self.p[x] != x {
            self.p[x] = self.find(self.p[x]);
        }
        self.p[x]
    }
    fn unite(&mut self, a: usize, b: usize) -> bool {
        let (mut a, mut b) = (self.find(a), self.find(b));
        if a == b {
            return false;
        }
        if self.r[a] < self.r[b] {
            std::mem::swap(&mut a, &mut b);
        }
        self.p[b] = a;
        if self.r[a] == self.r[b] {
            self.r[a] += 1;
        }
        true
    }
}

fn max_num_of_subsets(n: i32, edges: Vec<Vec<i32>>) -> i32 {
    let n = n as usize;
    let m = edges.len() as i32;
    let mut e3 = vec![];
    let mut e1 = vec![];
    let mut e2 = vec![];
    for e in edges {
        let u = e[1] as usize - 1;
        let v = e[2] as usize - 1;
        match e[0] {
            3 => e3.push((u, v)),
            1 => e1.push((u, v)),
            _ => e2.push((u, v)),
        }
    }
    let mut used = 0i32;
    let mut alice = Dsu::new(n);
    let mut bob = Dsu::new(n);
    for &(u, v) in &e3 {
        if alice.unite(u, v) {
            used += 1;
        }
        if bob.unite(u, v) {
            used += 1;
        }
    }
    for &(u, v) in &e1 {
        if alice.unite(u, v) {
            used += 1;
        }
    }
    for &(u, v) in &e2 {
        if bob.unite(u, v) {
            used += 1;
        }
    }
    m - used
}

fn main() {
    println!(
        "{}",
        max_num_of_subsets(
            4,
            vec![
                vec![3, 1, 2],
                vec![3, 2, 3],
                vec![1, 1, 3],
                vec![1, 2, 1],
                vec![1, 3, 1],
                vec![2, 4, 3],
            ],
        )
    );
}

#[cfg(test)]
mod tests {
    use super::max_num_of_subsets;

    #[test]
    fn example_one() {
        assert_eq!(
            max_num_of_subsets(
                4,
                vec![
                    vec![3, 1, 2],
                    vec![3, 2, 3],
                    vec![1, 1, 3],
                    vec![1, 2, 1],
                    vec![1, 3, 1],
                    vec![2, 4, 3],
                ],
            ),
            1
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(
            max_num_of_subsets(4, vec![vec![3, 1, 2], vec![3, 2, 3], vec![1, 1, 2], vec![2, 3, 3]]),
            0
        );
    }
}
