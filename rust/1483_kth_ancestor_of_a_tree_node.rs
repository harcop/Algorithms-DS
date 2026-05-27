/// LeetCode #1483 - Kth Ancestor Of A Tree Node
pub struct TreeAncestor {
    up: Vec<Vec<i32>>,
}
impl TreeAncestor {
    fn new(n: i32, parent: Vec<i32>) -> Self {
        let n = n as usize;
        let log = 17;
        let mut up = vec![vec![-1; log]; n];
        for i in 0..n { up[i][0] = parent[i]; }
        for j in 1..log {
            for i in 0..n {
                let p = up[i][j - 1];
                up[i][j] = if p == -1 { -1 } else { up[p as usize][j - 1] };
            }
        }
        TreeAncestor { up }
    }
    fn get_kth_ancestor(&self, node: i32, k: i32) -> i32 {
        let mut node = node as usize;
        for j in 0..17 {
            if k & (1 << j) != 0 {
                let p = self.up[node][j];
                if p == -1 { return -1; }
                node = p as usize;
            }
        }
        node as i32
    }
}
fn main() {
    let t = TreeAncestor::new(7, vec![-1,0,0,1,1,2,2]);
    println!("{}", t.get_kth_ancestor(3, 1));
}
#[cfg(test)]
mod tests {
    use super::TreeAncestor;
    #[test]
    fn example_one() {
        let t = TreeAncestor::new(7, vec![-1,0,0,1,1,2,2]);
        assert_eq!(t.get_kth_ancestor(3, 1), 1);
        assert_eq!(t.get_kth_ancestor(5, 2), 0);
        assert_eq!(t.get_kth_ancestor(6, 3), -1);
    }
}