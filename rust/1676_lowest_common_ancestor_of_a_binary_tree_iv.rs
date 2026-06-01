/// LeetCode #1676 - Lowest Common Ancestor Of A Binary Tree Iv
use std::collections::HashSet;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Box<TreeNode>>,
    pub right: Option<Box<TreeNode>>,
}

fn lowest_common_ancestor(root: &Option<Box<TreeNode>>, nodes: Vec<Option<Box<TreeNode>>>) -> Option<Box<TreeNode>> {
    let targets: HashSet<i32> = nodes.iter().filter_map(|n| n.as_ref().map(|x| x.val)).collect();
    fn dfs(r: &Option<Box<TreeNode>>, t: &HashSet<i32>) -> (i32, Option<i32>) {
        let Some(n) = r else { return (0, None); };
        let (lc, ll) = dfs(&n.left, t);
        let (rc, rl) = dfs(&n.right, t);
        let mut cnt = lc + rc + if t.contains(&n.val) { 1 } else { 0 };
        if ll.is_some() { return (cnt, ll); }
        if rl.is_some() { return (cnt, rl); }
        if cnt == t.len() as i32 { return (cnt, Some(n.val)); }
        (cnt, None)
    }
    let val = dfs(root, &targets).1?;
    Some(Box::new(TreeNode { val, left: None, right: None }))
}
fn main() { println!("{:?}", lowest_common_ancestor(&None, vec![])); }
#[cfg(test)]
mod tests {
    use super::{lowest_common_ancestor, TreeNode};
    fn node(v: i32) -> Option<Box<TreeNode>> {
        Some(Box::new(TreeNode { val: v, left: None, right: None }))
    }
    #[test]
    fn example_one() {
        let root = Some(Box::new(TreeNode {
            val: 3,
            left: Some(Box::new(TreeNode { val: 5, left: Some(Box::new(TreeNode { val: 6, left: None, right: None })), right: Some(Box::new(TreeNode { val: 2, left: None, right: None })) })),
            right: Some(Box::new(TreeNode { val: 1, left: Some(Box::new(TreeNode { val: 0, left: None, right: None })), right: Some(Box::new(TreeNode { val: 8, left: None, right: None })) })),
        }));
        let r = lowest_common_ancestor(&root, vec![node(6), node(2), node(8)]);
        assert_eq!(r.unwrap().val, 3);
    }
}