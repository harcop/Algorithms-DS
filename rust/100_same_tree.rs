/// LeetCode #100 - Same Tree
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

fn is_same_tree(p: Option<Box<TreeNode>>, q: Option<Box<TreeNode>>) -> bool {
    match (p, q) {
        (None, None) => true,
        (Some(a), Some(b)) => {
            a.val == b.val
                && is_same_tree(a.left, b.left)
                && is_same_tree(a.right, b.right)
        }
        _ => false,
    }
}

fn main() {
    let p = Some(Box::new(TreeNode::new(1)));
    let q = Some(Box::new(TreeNode::new(1)));
    println!("{}", is_same_tree(p, q));
}

#[cfg(test)]
mod tests {
    use super::{is_same_tree, TreeNode};

    #[test]
    fn example_one() {
        let mut p = Box::new(TreeNode::new(1));
        p.left = Some(Box::new(TreeNode::new(2)));
        p.right = Some(Box::new(TreeNode::new(3)));

        let mut q = Box::new(TreeNode::new(1));
        q.left = Some(Box::new(TreeNode::new(2)));
        q.right = Some(Box::new(TreeNode::new(3)));

        assert!(is_same_tree(Some(p), Some(q)));
    }

    #[test]
    fn example_two() {
        let mut p = Box::new(TreeNode::new(1));
        p.left = Some(Box::new(TreeNode::new(2)));

        let mut q = Box::new(TreeNode::new(1));
        q.right = Some(Box::new(TreeNode::new(2)));

        assert!(!is_same_tree(Some(p), Some(q)));
    }
}
