/// LeetCode #684 - Redundant Connection
struct DSU { p: Vec<usize> }
impl DSU {
    fn new(n: usize) -> Self { Self { p: (0..n).collect() } }
    fn find(&mut self, x: usize) -> usize { if self.p[x] != x { let r = self.find(self.p[x]); self.p[x] = r; } self.p[x] }
    fn union(&mut self, a: usize, b: usize) -> bool {
        let a = self.find(a); let b = self.find(b);
        if a == b { return false; }
        self.p[b] = a; true
    }
}

fn find_redundant_connection(edges: Vec<Vec<i32>>) -> Vec<i32> {
    let n = edges.len();
    let mut dsu = DSU::new(n + 1);
    for e in edges {
        if !dsu.union(e[0] as usize, e[1] as usize) { return e; }
    }
    vec![]
}

fn main() {
    println!("{:?}", find_redundant_connection(vec![vec![1,2],vec![1,3],vec![2,3]]));
}

#[cfg(test)]
mod tests {
    use super::find_redundant_connection;

    #[test]
    fn example_one() {
        assert_eq!(find_redundant_connection(vec![vec![1,2],vec![1,3],vec![2,3]]), vec![2,3]);
    }
}
