/// LeetCode #515 - Find Largest Value in Each Tree Row
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

fn largest_values(root: Option<Box<TreeNode>>) -> Vec<i32> {
    let mut ans = Vec::new();
    let mut q = VecDeque::new();
    if let Some(r) = root {
        q.push_back(r);
    }
    while !q.is_empty() {
        let mut mx = i32::MIN;
        for _ in 0..q.len() {
            let n = q.pop_front().unwrap();
            mx = mx.max(n.val);
            if let Some(l) = n.left {
                q.push_back(l);
            }
            if let Some(r) = n.right {
                q.push_back(r);
            }
        }
        ans.push(mx);
    }
    ans
}

fn main() {
    let mut root = Box::new(TreeNode::new(1));
    root.left = Some(Box::new(TreeNode::new(3)));
    root.right = Some(Box::new(TreeNode::new(2)));
    println!("{:?}", largest_values(Some(root)));
}

#[cfg(test)]
mod tests {
    use super::{largest_values, TreeNode};

    #[test]
    fn example_one() {
        let mut root = Box::new(TreeNode::new(1));
        let mut l = Box::new(TreeNode::new(3));
        l.left = Some(Box::new(TreeNode::new(5)));
        l.right = Some(Box::new(TreeNode::new(3)));
        root.left = Some(l);
        let mut r = Box::new(TreeNode::new(2));
        r.right = Some(Box::new(TreeNode::new(9)));
        root.right = Some(r);
        assert_eq!(largest_values(Some(root)), vec![1, 3, 9]);
    }

    #[test]
    fn example_two() {
        let mut root = Box::new(TreeNode::new(1));
        root.left = Some(Box::new(TreeNode::new(2)));
        root.right = Some(Box::new(TreeNode::new(3)));
        assert_eq!(largest_values(Some(root)), vec![1, 3]);
    }
}
