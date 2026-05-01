/// LeetCode #124 - Binary Tree Maximum Path Sum
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

fn max_path_sum(root: Option<Box<TreeNode>>) -> i32 {
    let mut best = i32::MIN;
    fn dfs(node: &Option<Box<TreeNode>>, best: &mut i32) -> i32 {
        match node {
            None => 0,
            Some(n) => {
                let left_gain = dfs(&n.left, best).max(0);
                let right_gain = dfs(&n.right, best).max(0);
                let path_through = n.val + left_gain + right_gain;
                *best = (*best).max(path_through);
                n.val + left_gain.max(right_gain)
            }
        }
    }
    dfs(&root, &mut best);
    best
}

fn main() {
    let mut r = Box::new(TreeNode::new(1));
    r.left = Some(Box::new(TreeNode::new(2)));
    r.right = Some(Box::new(TreeNode::new(3)));
    println!("{}", max_path_sum(Some(r)));
}

#[cfg(test)]
mod tests {
    use super::{max_path_sum, TreeNode};

    #[test]
    fn example_one() {
        let mut r = Box::new(TreeNode::new(1));
        r.left = Some(Box::new(TreeNode::new(2)));
        r.right = Some(Box::new(TreeNode::new(3)));
        assert_eq!(max_path_sum(Some(r)), 6);
    }

    #[test]
    fn example_two() {
        let mut r = Box::new(TreeNode::new(-10));
        r.left = Some(Box::new(TreeNode::new(9)));
        r.right = Some(Box::new(TreeNode::new(20)));
        r.right.as_mut().unwrap().left = Some(Box::new(TreeNode::new(15)));
        r.right.as_mut().unwrap().right = Some(Box::new(TreeNode::new(7)));
        assert_eq!(max_path_sum(Some(r)), 42);
    }
}
