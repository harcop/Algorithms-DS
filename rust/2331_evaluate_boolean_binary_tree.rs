/// LeetCode #2331 - Evaluate Boolean Binary Tree
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

fn evaluate_tree(root: &Option<Box<TreeNode>>) -> bool {
    let Some(node) = root else {
        return false;
    };
    if node.left.is_none() {
        return node.val == 1;
    }
    if node.val == 2 {
        return evaluate_tree(&node.left) || evaluate_tree(&node.right);
    }
    evaluate_tree(&node.left) && evaluate_tree(&node.right)
}

fn main() {
    let mut and = TreeNode::new(3);
    and.left = Some(Box::new(TreeNode::new(0)));
    and.right = Some(Box::new(TreeNode::new(1)));
    let mut root = TreeNode::new(2);
    root.left = Some(Box::new(TreeNode::new(1)));
    root.right = Some(Box::new(and));
    println!("{}", evaluate_tree(&Some(Box::new(root))));
}

#[cfg(test)]
mod tests {
    use super::{evaluate_tree, TreeNode};

    #[test]
    fn example_one() {
        let mut and = TreeNode::new(3);
        and.left = Some(Box::new(TreeNode::new(0)));
        and.right = Some(Box::new(TreeNode::new(1)));
        let mut root = TreeNode::new(2);
        root.left = Some(Box::new(TreeNode::new(1)));
        root.right = Some(Box::new(and));
        assert!(evaluate_tree(&Some(Box::new(root))));
    }

    #[test]
    fn example_two() {
        let root = Some(Box::new(TreeNode::new(0)));
        assert!(!evaluate_tree(&root));
    }
}
