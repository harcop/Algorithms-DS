/// LeetCode #145 - Binary Tree Postorder Traversal
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

fn postorder_traversal(root: Option<Box<TreeNode>>) -> Vec<i32> {
    let mut out = Vec::new();
    fn dfs(node: &Option<Box<TreeNode>>, out: &mut Vec<i32>) {
        if let Some(n) = node {
            dfs(&n.left, out);
            dfs(&n.right, out);
            out.push(n.val);
        }
    }
    dfs(&root, &mut out);
    out
}

fn main() {
    let mut r = Box::new(TreeNode::new(1));
    r.right = Some(Box::new(TreeNode::new(2)));
    r.right.as_mut().unwrap().left = Some(Box::new(TreeNode::new(3)));
    println!("{:?}", postorder_traversal(Some(r)));
}

#[cfg(test)]
mod tests {
    use super::{postorder_traversal, TreeNode};

    #[test]
    fn example_one() {
        let mut r = Box::new(TreeNode::new(1));
        r.right = Some(Box::new(TreeNode::new(2)));
        r.right.as_mut().unwrap().left = Some(Box::new(TreeNode::new(3)));
        assert_eq!(postorder_traversal(Some(r)), vec![3, 2, 1]);
    }

    #[test]
    fn example_two() {
        assert_eq!(postorder_traversal(None), vec![]);
    }
}
