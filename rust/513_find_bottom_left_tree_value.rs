/// LeetCode #513 - Find Bottom Left Tree Value
use std::collections::VecDeque;

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

fn find_bottom_left_value(root: Option<Box<TreeNode>>) -> i32 {
    let mut q = VecDeque::new();
    q.push_back(root.unwrap());
    let mut ans = 0;
    while !q.is_empty() {
        ans = q[0].val;
        for _ in 0..q.len() {
            let n = q.pop_front().unwrap();
            if let Some(l) = n.left {
                q.push_back(l);
            }
            if let Some(r) = n.right {
                q.push_back(r);
            }
        }
    }
    ans
}

fn main() {
    let mut root = Box::new(TreeNode::new(2));
    root.left = Some(Box::new(TreeNode::new(1)));
    root.right = Some(Box::new(TreeNode::new(3)));
    println!("{}", find_bottom_left_value(Some(root)));
}

#[cfg(test)]
mod tests {
    use super::{find_bottom_left_value, TreeNode};

    #[test]
    fn example_one() {
        let mut root = Box::new(TreeNode::new(2));
        root.left = Some(Box::new(TreeNode::new(1)));
        root.right = Some(Box::new(TreeNode::new(3)));
        assert_eq!(find_bottom_left_value(Some(root)), 1);
    }

    #[test]
    fn example_two() {
        let mut root = Box::new(TreeNode::new(1));
        let mut l = Box::new(TreeNode::new(2));
        l.left = Some(Box::new(TreeNode::new(4)));
        let mut r = Box::new(TreeNode::new(3));
        let mut r_l = Box::new(TreeNode::new(5));
        r_l.left = Some(Box::new(TreeNode::new(7)));
        r.left = Some(r_l);
        r.right = Some(Box::new(TreeNode::new(6)));
        root.left = Some(l);
        root.right = Some(r);
        assert_eq!(find_bottom_left_value(Some(root)), 7);
    }
}
