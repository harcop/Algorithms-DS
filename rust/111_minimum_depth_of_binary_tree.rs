/// LeetCode #111 - Minimum Depth of Binary Tree
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

fn min_depth(root: Option<Box<TreeNode>>) -> i32 {
    fn dfs(node: &Option<Box<TreeNode>>) -> i32 {
        match node {
            None => 0,
            Some(n) => match (&n.left, &n.right) {
                (None, None) => 1,
                (Some(_), None) => 1 + dfs(&n.left),
                (None, Some(_)) => 1 + dfs(&n.right),
                _ => 1 + dfs(&n.left).min(dfs(&n.right)),
            },
        }
    }
    dfs(&root)
}

fn main() {
    let mut root = Box::new(TreeNode::new(3));
    root.left = Some(Box::new(TreeNode::new(9)));
    root.right = Some(Box::new(TreeNode::new(20)));
    println!("{}", min_depth(Some(root)));
}

#[cfg(test)]
mod tests {
    use super::{min_depth, TreeNode};

    #[test]
    fn example_one() {
        let mut root = Box::new(TreeNode::new(3));
        root.left = Some(Box::new(TreeNode::new(9)));
        root.right = Some(Box::new(TreeNode::new(20)));
        root.right.as_mut().unwrap().left = Some(Box::new(TreeNode::new(15)));
        root.right.as_mut().unwrap().right = Some(Box::new(TreeNode::new(7)));
        assert_eq!(min_depth(Some(root)), 2);
    }

    #[test]
    fn example_two() {
        let mut root = Box::new(TreeNode::new(2));
        root.right = Some(Box::new(TreeNode::new(3)));
        root.right.as_mut().unwrap().right = Some(Box::new(TreeNode::new(4)));
        root.right.as_mut().unwrap().right.as_mut().unwrap().right = Some(Box::new(TreeNode::new(5)));
        root.right.as_mut().unwrap().right.as_mut().unwrap().right.as_mut().unwrap().right =
            Some(Box::new(TreeNode::new(6)));
        assert_eq!(min_depth(Some(root)), 5);
    }
}
