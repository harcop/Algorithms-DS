/// LeetCode #104 - Maximum Depth of Binary Tree
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

fn max_depth(root: Option<Box<TreeNode>>) -> i32 {
    fn dfs(node: &Option<Box<TreeNode>>) -> i32 {
        match node {
            None => 0,
            Some(n) => 1 + dfs(&n.left).max(dfs(&n.right)),
        }
    }
    dfs(&root)
}

fn main() {
    let mut root = Box::new(TreeNode::new(3));
    root.left = Some(Box::new(TreeNode::new(9)));
    root.right = Some(Box::new(TreeNode::new(20)));
    println!("{}", max_depth(Some(root)));
}

#[cfg(test)]
mod tests {
    use super::{max_depth, TreeNode};

    #[test]
    fn example_one() {
        let mut root = Box::new(TreeNode::new(3));
        root.left = Some(Box::new(TreeNode::new(9)));
        root.right = Some(Box::new(TreeNode::new(20)));
        root.right.as_mut().unwrap().left = Some(Box::new(TreeNode::new(15)));
        root.right.as_mut().unwrap().right = Some(Box::new(TreeNode::new(7)));
        assert_eq!(max_depth(Some(root)), 3);
    }

    #[test]
    fn example_two() {
        assert_eq!(max_depth(None), 0);
    }
}
