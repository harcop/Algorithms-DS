/// LeetCode #101 - Symmetric Tree
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

fn is_symmetric(root: Option<Box<TreeNode>>) -> bool {
    fn mirror(a: &Option<Box<TreeNode>>, b: &Option<Box<TreeNode>>) -> bool {
        match (a, b) {
            (None, None) => true,
            (Some(x), Some(y)) => {
                x.val == y.val
                    && mirror(&x.left, &y.right)
                    && mirror(&x.right, &y.left)
            }
            _ => false,
        }
    }
    match root {
        Some(r) => mirror(&r.left, &r.right),
        None => true,
    }
}

fn main() {
    let mut root = Box::new(TreeNode::new(1));
    root.left = Some(Box::new(TreeNode::new(2)));
    root.right = Some(Box::new(TreeNode::new(2)));
    println!("{}", is_symmetric(Some(root)));
}

#[cfg(test)]
mod tests {
    use super::{is_symmetric, TreeNode};

    #[test]
    fn example_one() {
        let mut root = Box::new(TreeNode::new(1));
        root.left = Some(Box::new(TreeNode::new(2)));
        root.right = Some(Box::new(TreeNode::new(2)));
        root.left.as_mut().unwrap().left = Some(Box::new(TreeNode::new(3)));
        root.left.as_mut().unwrap().right = Some(Box::new(TreeNode::new(4)));
        root.right.as_mut().unwrap().left = Some(Box::new(TreeNode::new(4)));
        root.right.as_mut().unwrap().right = Some(Box::new(TreeNode::new(3)));
        assert!(is_symmetric(Some(root)));
    }

    #[test]
    fn example_two() {
        let mut root = Box::new(TreeNode::new(1));
        root.left = Some(Box::new(TreeNode::new(2)));
        root.right = Some(Box::new(TreeNode::new(2)));
        root.right.as_mut().unwrap().right = Some(Box::new(TreeNode::new(3)));
        assert!(!is_symmetric(Some(root)));
    }
}
