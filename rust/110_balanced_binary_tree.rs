/// LeetCode #110 - Balanced Binary Tree
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

fn is_balanced(root: Option<Box<TreeNode>>) -> bool {
    fn height(node: &Option<Box<TreeNode>>) -> i32 {
        match node {
            None => 0,
            Some(n) => {
                let lh = height(&n.left);
                if lh == -1 {
                    return -1;
                }
                let rh = height(&n.right);
                if rh == -1 {
                    return -1;
                }
                if (lh - rh).abs() > 1 {
                    return -1;
                }
                1 + lh.max(rh)
            }
        }
    }
    height(&root) != -1
}

fn main() {
    let mut root = Box::new(TreeNode::new(3));
    root.left = Some(Box::new(TreeNode::new(9)));
    root.right = Some(Box::new(TreeNode::new(20)));
    println!("{}", is_balanced(Some(root)));
}

#[cfg(test)]
mod tests {
    use super::{is_balanced, TreeNode};

    #[test]
    fn example_one() {
        let mut root = Box::new(TreeNode::new(3));
        root.left = Some(Box::new(TreeNode::new(9)));
        root.right = Some(Box::new(TreeNode::new(20)));
        root.right.as_mut().unwrap().left = Some(Box::new(TreeNode::new(15)));
        root.right.as_mut().unwrap().right = Some(Box::new(TreeNode::new(7)));
        assert!(is_balanced(Some(root)));
    }

    #[test]
    fn example_two() {
        let mut root = Box::new(TreeNode::new(1));
        root.left = Some(Box::new(TreeNode::new(2)));
        root.right = Some(Box::new(TreeNode::new(3)));
        root.left.as_mut().unwrap().left = Some(Box::new(TreeNode::new(4)));
        root.left.as_mut().unwrap().right = Some(Box::new(TreeNode::new(5)));
        root.left.as_mut().unwrap().left.as_mut().unwrap().left = Some(Box::new(TreeNode::new(6)));
        assert!(!is_balanced(Some(root)));
    }
}
