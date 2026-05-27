/// LeetCode #1457 - Pseudo Palindromic Paths In A Binary Tree
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Box<TreeNode>>,
    pub right: Option<Box<TreeNode>>,
}
fn pseudo_palindromic_paths(root: Option<Box<TreeNode>>) -> i32 {
    fn dfs(node: Option<Box<TreeNode>>, mask: u32) -> i32 {
        match node {
            None => 0,
            Some(n) => {
                let m = mask ^ (1 << n.val);
                if n.left.is_none() && n.right.is_none() {
                    return if m.count_ones() <= 1 { 1 } else { 0 };
                }
                dfs(n.left, m) + dfs(n.right, m)
            }
        }
    }
    dfs(root, 0)
}
fn main() {
    let mut r = Box::new(TreeNode { val: 2, left: None, right: None });
    r.left = Some(Box::new(TreeNode { val: 3, left: None, right: None }));
    r.right = Some(Box::new(TreeNode { val: 1, left: None, right: None }));
    println!("{}", pseudo_palindromic_paths(Some(r)));
}
#[cfg(test)]
mod tests {
    use super::{pseudo_palindromic_paths, TreeNode};
    #[test]
    fn example_one() {
        let mut r = Box::new(TreeNode { val: 2, left: None, right: None });
        r.left = Some(Box::new(TreeNode { val: 3, left: None, right: None }));
        r.right = Some(Box::new(TreeNode { val: 1, left: None, right: None }));
        r.left.as_mut().unwrap().left = Some(Box::new(TreeNode { val: 3, left: None, right: None }));
        r.left.as_mut().unwrap().right = Some(Box::new(TreeNode { val: 1, left: None, right: None }));
        r.right.as_mut().unwrap().left = Some(Box::new(TreeNode { val: 2, left: None, right: None }));
        assert_eq!(pseudo_palindromic_paths(Some(r)), 2);
    }
}