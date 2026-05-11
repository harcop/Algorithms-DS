/// LeetCode #653 - Two Sum IV - Input is a BST
use std::collections::HashSet;

#[derive(Debug)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Box<TreeNode>>,
    pub right: Option<Box<TreeNode>>,
}

fn find_target(root: Option<Box<TreeNode>>, k: i32) -> bool {
    let mut seen: HashSet<i32> = HashSet::new();
    fn dfs(node: &Option<Box<TreeNode>>, k: i32, seen: &mut HashSet<i32>) -> bool {
        let Some(n) = node else { return false };
        if seen.contains(&(k - n.val)) { return true; }
        seen.insert(n.val);
        dfs(&n.left, k, seen) || dfs(&n.right, k, seen)
    }
    dfs(&root, k, &mut seen)
}

fn main() {
    println!("{}", find_target(None, 5));
}

#[cfg(test)]
mod tests {
    use super::{find_target, TreeNode};

    #[test]
    fn example_one() {
        let root = Some(Box::new(TreeNode {
            val: 5,
            left: Some(Box::new(TreeNode {
                val: 3,
                left: Some(Box::new(TreeNode { val: 2, left: None, right: None })),
                right: Some(Box::new(TreeNode { val: 4, left: None, right: None })),
            })),
            right: Some(Box::new(TreeNode {
                val: 6,
                left: None,
                right: Some(Box::new(TreeNode { val: 7, left: None, right: None })),
            })),
        }));
        assert!(find_target(root, 9));
    }
}
