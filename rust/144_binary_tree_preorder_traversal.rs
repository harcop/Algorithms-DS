/// LeetCode #144 - Binary Tree Preorder Traversal
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

fn preorder_traversal(root: Option<Box<TreeNode>>) -> Vec<i32> {
    let mut out = Vec::new();
    fn dfs(node: &Option<Box<TreeNode>>, out: &mut Vec<i32>) {
        if let Some(n) = node {
            out.push(n.val);
            dfs(&n.left, out);
            dfs(&n.right, out);
        }
    }
    dfs(&root, &mut out);
    out
}

fn main() {
    let mut r = Box::new(TreeNode::new(1));
    r.right = Some(Box::new(TreeNode::new(2)));
    r.right.as_mut().unwrap().left = Some(Box::new(TreeNode::new(3)));
    println!("{:?}", preorder_traversal(Some(r)));
}

#[cfg(test)]
mod tests {
    use super::{preorder_traversal, TreeNode};

    #[test]
    fn example_one() {
        let mut r = Box::new(TreeNode::new(1));
        r.right = Some(Box::new(TreeNode::new(2)));
        r.right.as_mut().unwrap().left = Some(Box::new(TreeNode::new(3)));
        assert_eq!(preorder_traversal(Some(r)), vec![1, 2, 3]);
    }

    #[test]
    fn example_two() {
        assert_eq!(preorder_traversal(None), vec![]);
    }
}
