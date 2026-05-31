/// LeetCode #1627 - Graph Connectivity With Threshold
struct Dsu {
    p: Vec<usize>,
    sz: Vec<usize>,
}
impl Dsu {
    fn new(n: usize) -> Self { Dsu { p: (0..n).collect(), sz: vec![1; n] } }
    fn find(&mut self, x: usize) -> usize {
        if self.p[x] != x { self.p[x] = self.find(self.p[x]); }
        self.p[x]
    }
    fn union(&mut self, a: usize, b: usize) -> bool {
        let (a, b) = (self.find(a), self.find(b));
        if a == b { return false; }
        if self.sz[a] < self.sz[b] { return self.union(b, a); }
        self.p[b] = a;
        self.sz[a] += self.sz[b];
        true
    }
}

fn are_connected(n: i32, threshold: i32, queries: Vec<Vec<i32>>) -> Vec<bool> {
    let n = n as usize;
    let mut dsu = Dsu::new(n + 1);
    for z in (threshold + 1)..=n as i32 {
        let z = z as usize;
        for x in (z + z..=n).step_by(z) {
            dsu.union(z, x);
        }
    }
    queries.into_iter().map(|q| dsu.find(q[0] as usize) == dsu.find(q[1] as usize)).collect()
}
fn main() { println!("{:?}", are_connected(6, 2, vec![vec![1,4],vec![2,5],vec![3,6]])); }
#[cfg(test)]
mod tests {
    use super::are_connected;
    #[test]
    fn example_one() { assert_eq!(are_connected(6, 2, vec![vec![1,4],vec![2,5],vec![3,6]]), vec![false,false,true]); }
}