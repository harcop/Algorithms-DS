use std::collections::VecDeque;

/// LeetCode #102 - Binary Tree Level Order Traversal
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

fn level_order(root: Option<Box<TreeNode>>) -> Vec<Vec<i32>> {
    let mut out = Vec::new();
    let mut q = VecDeque::new();
    if root.is_none() {
        return out;
    }
    q.push_back(root);

    while !q.is_empty() {
        let sz = q.len();
        let mut level = Vec::with_capacity(sz);
        for _ in 0..sz {
            let node = q.pop_front().unwrap().unwrap();
            level.push(node.val);
            if let Some(l) = node.left {
                q.push_back(Some(l));
            }
            if let Some(r) = node.right {
                q.push_back(Some(r));
            }
        }
        out.push(level);
    }
    out
}

fn main() {
    let mut root = Box::new(TreeNode::new(3));
    root.left = Some(Box::new(TreeNode::new(9)));
    root.right = Some(Box::new(TreeNode::new(20)));
    root.right.as_mut().unwrap().left = Some(Box::new(TreeNode::new(15)));
    root.right.as_mut().unwrap().right = Some(Box::new(TreeNode::new(7)));
    println!("{:?}", level_order(Some(root)));
}

#[cfg(test)]
mod tests {
    use super::{level_order, TreeNode};

    #[test]
    fn example_one() {
        let mut root = Box::new(TreeNode::new(3));
        root.left = Some(Box::new(TreeNode::new(9)));
        root.right = Some(Box::new(TreeNode::new(20)));
        root.right.as_mut().unwrap().left = Some(Box::new(TreeNode::new(15)));
        root.right.as_mut().unwrap().right = Some(Box::new(TreeNode::new(7)));
        assert_eq!(level_order(Some(root)), vec![vec![3], vec![9, 20], vec![15, 7]]);
    }

    #[test]
    fn example_two() {
        assert_eq!(level_order(None), Vec::<Vec<i32>>::new());
    }
}
