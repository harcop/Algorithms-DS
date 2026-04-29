/// LeetCode #94 - Binary Tree Inorder Traversal
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

fn inorder_traversal(root: Option<Box<TreeNode>>) -> Vec<i32> {
    let mut out = Vec::new();
    fn dfs(node: &Option<Box<TreeNode>>, out: &mut Vec<i32>) {
        if let Some(n) = node {
            dfs(&n.left, out);
            out.push(n.val);
            dfs(&n.right, out);
        }
    }
    dfs(&root, &mut out);
    out
}

fn main() {
    let mut root = Box::new(TreeNode::new(1));
    root.right = Some(Box::new(TreeNode::new(2)));
    root.right.as_mut().unwrap().left = Some(Box::new(TreeNode::new(3)));
    println!("{:?}", inorder_traversal(Some(root)));
}

#[cfg(test)]
mod tests {
    use super::{inorder_traversal, TreeNode};

    #[test]
    fn example_one() {
        let mut root = Box::new(TreeNode::new(1));
        root.right = Some(Box::new(TreeNode::new(2)));
        root.right.as_mut().unwrap().left = Some(Box::new(TreeNode::new(3)));
        assert_eq!(inorder_traversal(Some(root)), vec![1, 3, 2]);
    }

    #[test]
    fn example_two() {
        assert_eq!(inorder_traversal(None), vec![]);
    }
}
