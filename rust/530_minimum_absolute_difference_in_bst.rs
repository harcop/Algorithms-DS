/// LeetCode #530 - Minimum Absolute Difference in BST
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

fn get_minimum_difference(root: Option<Box<TreeNode>>) -> i32 {
    let mut prev: Option<i32> = None;
    let mut ans = i32::MAX;
    fn inorder(node: &Option<Box<TreeNode>>, prev: &mut Option<i32>, ans: &mut i32) {
        if let Some(n) = node {
            inorder(&n.left, prev, ans);
            if let Some(p) = *prev {
                *ans = (*ans).min(n.val - p);
            }
            *prev = Some(n.val);
            inorder(&n.right, prev, ans);
        }
    }
    inorder(&root, &mut prev, &mut ans);
    ans
}

fn main() {
    let mut root = Box::new(TreeNode::new(4));
    root.left = Some(Box::new(TreeNode::new(2)));
    root.right = Some(Box::new(TreeNode::new(6)));
    println!("{}", get_minimum_difference(Some(root)));
}

#[cfg(test)]
mod tests {
    use super::{get_minimum_difference, TreeNode};

    #[test]
    fn example_one() {
        let mut root = Box::new(TreeNode::new(4));
        let mut left = Box::new(TreeNode::new(2));
        left.left = Some(Box::new(TreeNode::new(1)));
        left.right = Some(Box::new(TreeNode::new(3)));
        root.left = Some(left);
        root.right = Some(Box::new(TreeNode::new(6)));
        assert_eq!(get_minimum_difference(Some(root)), 1);
    }

    #[test]
    fn example_two() {
        let mut root = Box::new(TreeNode::new(1));
        root.left = Some(Box::new(TreeNode::new(0)));
        let mut right = Box::new(TreeNode::new(48));
        right.left = Some(Box::new(TreeNode::new(12)));
        right.right = Some(Box::new(TreeNode::new(49)));
        root.right = Some(right);
        assert_eq!(get_minimum_difference(Some(root)), 1);
    }
}
