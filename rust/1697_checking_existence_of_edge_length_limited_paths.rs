/// LeetCode #1697 - Checking Existence Of Edge Length Limited Paths
struct Dsu {
    p: Vec<usize>,
}
impl Dsu {
    fn new(n: usize) -> Self { Self { p: (0..n).collect() } }
    fn find(&mut self, x: usize) -> usize {
        if self.p[x] != x { self.p[x] = self.find(self.p[x]); }
        self.p[x]
    }
    fn unite(&mut self, a: usize, b: usize) {
        let (a, b) = (self.find(a), self.find(b));
        if a != b { self.p[a] = b; }
    }
}

fn distance_limited_paths_exist(n: i32, edge_list: Vec<Vec<i32>>, queries: Vec<Vec<i32>>) -> Vec<bool> {
    let n = n as usize;
    let mut edges: Vec<(i32, usize, usize)> = edge_list.iter().map(|e| (e[2], e[0] as usize, e[1] as usize)).collect();
    edges.sort_unstable();
    let mut qs: Vec<(i32, usize, usize, usize)> = queries.iter().enumerate().map(|(i, q)| (q[2], q[0] as usize, q[1] as usize, i)).collect();
    qs.sort_unstable();
    let mut dsu = Dsu::new(n);
    let mut ans = vec![false; queries.len()];
    let mut ei = 0usize;
    for (limit, p, q, idx) in qs {
        while ei < edges.len() && edges[ei].0 < limit {
            dsu.unite(edges[ei].1, edges[ei].2);
            ei += 1;
        }
        ans[idx] = dsu.find(p) == dsu.find(q);
    }
    ans
}
fn main() { println!("{:?}", distance_limited_paths_exist(3, vec![vec![0,1,2],vec![1,2,4],vec![2,0,8],vec![1,0,16]], vec![vec![0,1,2],vec![0,2,6]])); }
#[cfg(test)]
mod tests {
    use super::distance_limited_paths_exist;
    #[test]
    fn example_one() {
        assert_eq!(distance_limited_paths_exist(3, vec![vec![0,1,2],vec![1,2,4],vec![2,0,8],vec![1,0,16]], vec![vec![0,1,2],vec![0,2,6]]), vec![false, true]);
    }
}