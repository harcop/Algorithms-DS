/// LeetCode #3608 - Minimum Time for K Connected Components
struct UnionFind {
    p: Vec<i32>,
    sz: Vec<i32>,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        Self {
            p: (0..n as i32).collect(),
            sz: vec![1; n],
        }
    }
    fn find(&mut self, x: i32) -> i32 {
        let i = x as usize;
        if self.p[i] != x {
            self.p[i] = self.find(self.p[i]);
        }
        self.p[i]
    }
    fn union(&mut self, a: i32, b: i32) -> bool {
        let (pa, pb) = (self.find(a), self.find(b));
        if pa == pb {
            return false;
        }
        let (a, b) = (pa as usize, pb as usize);
        if self.sz[a] < self.sz[b] {
            self.p[a] = pb;
            self.sz[b] += self.sz[a];
        } else {
            self.p[b] = pa;
            self.sz[a] += self.sz[b];
        }
        true
    }
}

fn min_time(n: i32, mut edges: Vec<Vec<i32>>, k: i32) -> i32 {
    edges.sort_by_key(|e| e[2]);
    let mut uf = UnionFind::new(n as usize);
    let mut cnt = n;
    for e in edges.iter().rev() {
        if uf.union(e[0], e[1]) {
            cnt -= 1;
            if cnt < k {
                return e[2];
            }
        }
    }
    0
}

fn main() {
    println!("{}", min_time(2, vec![vec![0, 1, 3]], 2));
}

#[cfg(test)]
mod tests {
    use super::min_time;

    #[test]
    fn example1() {
        assert_eq!(min_time(2, vec![vec![0, 1, 3]], 2), 3);
    }

    #[test]
    fn example2() {
        assert_eq!(min_time(3, vec![vec![0, 1, 2], vec![1, 2, 4]], 3), 4);
    }

    #[test]
    fn example3() {
        assert_eq!(min_time(3, vec![vec![0, 2, 5]], 2), 0);
    }
}
