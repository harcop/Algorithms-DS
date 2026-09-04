/// LeetCode #663 - Equal Tree Partition
use std::collections::HashSet;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Box<TreeNode>>,
    pub right: Option<Box<TreeNode>>,
}

impl TreeNode {
    fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

fn check_equal_tree(root: Option<Box<TreeNode>>) -> bool {
    let mut sums = HashSet::new();
    fn dfs(node: &Option<Box<TreeNode>>, sums: &mut HashSet<i64>, is_root: bool) -> i64 {
        let Some(n) = node else {
            return 0;
        };
        let s = n.val as i64 + dfs(&n.left, sums, false) + dfs(&n.right, sums, false);
        if !is_root {
            sums.insert(s);
        }
        s
    }
    let total = dfs(&root, &mut sums, true);
    total % 2 == 0 && sums.contains(&(total / 2))
}

fn main() {
    let mut root = Box::new(TreeNode::new(5));
    root.left = Some(Box::new(TreeNode::new(10)));
    println!("{}", check_equal_tree(Some(root)));
}

#[cfg(test)]
mod tests {
    use super::{check_equal_tree, TreeNode};

    #[test]
    fn example_one() {
        let mut root = Box::new(TreeNode::new(5));
        root.left = Some(Box::new(TreeNode::new(10)));
        let mut r = Box::new(TreeNode::new(10));
        r.left = Some(Box::new(TreeNode::new(2)));
        r.right = Some(Box::new(TreeNode::new(3)));
        root.right = Some(r);
        assert!(check_equal_tree(Some(root)));
    }

    #[test]
    fn example_two() {
        let mut root = Box::new(TreeNode::new(1));
        root.left = Some(Box::new(TreeNode::new(2)));
        let mut r = Box::new(TreeNode::new(10));
        r.left = Some(Box::new(TreeNode::new(2)));
        r.right = Some(Box::new(TreeNode::new(20)));
        root.right = Some(r);
        assert!(!check_equal_tree(Some(root)));
    }
}
