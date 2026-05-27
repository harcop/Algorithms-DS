/// LeetCode #1448 - Count Good Nodes In Binary Tree
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Box<TreeNode>>,
    pub right: Option<Box<TreeNode>>,
}
fn good_nodes(root: Option<Box<TreeNode>>) -> i32 {
    fn dfs(node: Option<Box<TreeNode>>, mx: i32) -> i32 {
        match node {
            None => 0,
            Some(n) => {
                let good = if n.val >= mx { 1 } else { 0 };
                let nm = mx.max(n.val);
                good + dfs(n.left, nm) + dfs(n.right, nm)
            }
        }
    }
    dfs(root, i32::MIN)
}
fn main() {
    let mut r = Box::new(TreeNode { val: 3, left: None, right: None });
    r.left = Some(Box::new(TreeNode { val: 1, left: None, right: None }));
    r.right = Some(Box::new(TreeNode { val: 4, left: None, right: None }));
    println!("{}", good_nodes(Some(r)));
}
#[cfg(test)]
mod tests {
    use super::{good_nodes, TreeNode};
    #[test]
    fn example_one() {
        let mut r = Box::new(TreeNode { val: 3, left: None, right: None });
        r.left = Some(Box::new(TreeNode { val: 1, left: None, right: None }));
        r.right = Some(Box::new(TreeNode { val: 4, left: None, right: None }));
        r.left.as_mut().unwrap().left = Some(Box::new(TreeNode { val: 3, left: None, right: None }));
        r.right.as_mut().unwrap().left = Some(Box::new(TreeNode { val: 1, left: None, right: None }));
        r.right.as_mut().unwrap().right = Some(Box::new(TreeNode { val: 5, left: None, right: None }));
        assert_eq!(good_nodes(Some(r)), 4);
    }
}