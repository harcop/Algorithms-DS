/// LeetCode #1530 - Number Of Good Leaf Nodes Pairs
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct TreeNode { pub val: i32, pub left: Option<Box<TreeNode>>, pub right: Option<Box<TreeNode>> }
fn count_pairs(root: Option<Box<TreeNode>>, distance: i32) -> i32 {
    fn dfs(node: Option<Box<TreeNode>>, dist: i32) -> (i32, Vec<i32>) {
        let Some(n) = node else { return (0, vec![]); };
        if n.left.is_none() && n.right.is_none() { return (0, vec![1]); }
        let (l_cnt, l_dist) = dfs(n.left, dist);
        let (r_cnt, r_dist) = dfs(n.right, dist);
        let mut cnt = l_cnt + r_cnt;
        for &ld in &l_dist { for &rd in &r_dist { if ld + rd <= dist { cnt += 1; } } }
        let mut merged = Vec::new();
        for d in l_dist.into_iter().chain(r_dist).filter_map(|d| if d + 1 <= dist { Some(d + 1) } else { None }) {
            merged.push(d);
        }
        (cnt, merged)
    }
    dfs(root, distance).0
}
fn main() {
    let mut r = Box::new(TreeNode { val: 1, left: None, right: None });
    r.left = Some(Box::new(TreeNode { val: 2, left: None, right: None }));
    r.right = Some(Box::new(TreeNode { val: 3, left: None, right: None }));
    println!("{}", count_pairs(Some(r), 3));
}
#[cfg(test)]
mod tests {
    use super::{count_pairs, TreeNode};
    #[test]
    fn example_one() {
        let mut r = Box::new(TreeNode { val: 1, left: None, right: None });
        r.left = Some(Box::new(TreeNode { val: 2, left: None, right: None }));
        r.right = Some(Box::new(TreeNode { val: 3, left: None, right: None }));
        assert_eq!(count_pairs(Some(r), 3), 1);
    }
}
