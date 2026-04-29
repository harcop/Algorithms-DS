/// LeetCode #98 - Validate Binary Search Tree
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

fn is_valid_bst(root: Option<Box<TreeNode>>) -> bool {
    fn validate(node: &Option<Box<TreeNode>>, lo: Option<i64>, hi: Option<i64>) -> bool {
        if let Some(n) = node {
            let v = n.val as i64;
            if let Some(l) = lo {
                if v <= l {
                    return false;
                }
            }
            if let Some(h) = hi {
                if v >= h {
                    return false;
                }
            }
            validate(&n.left, lo, Some(v)) && validate(&n.right, Some(v), hi)
        } else {
            true
        }
    }
    validate(&root, None, None)
}

fn main() {
    let mut root = Box::new(TreeNode::new(2));
    root.left = Some(Box::new(TreeNode::new(1)));
    root.right = Some(Box::new(TreeNode::new(3)));
    println!("{}", is_valid_bst(Some(root)));
}

#[cfg(test)]
mod tests {
    use super::{is_valid_bst, TreeNode};

    #[test]
    fn example_one() {
        let mut root = Box::new(TreeNode::new(2));
        root.left = Some(Box::new(TreeNode::new(1)));
        root.right = Some(Box::new(TreeNode::new(3)));
        assert!(is_valid_bst(Some(root)));
    }

    #[test]
    fn example_two() {
        let mut root = Box::new(TreeNode::new(5));
        root.left = Some(Box::new(TreeNode::new(1)));
        root.right = Some(Box::new(TreeNode::new(4)));
        root.right.as_mut().unwrap().left = Some(Box::new(TreeNode::new(3)));
        root.right.as_mut().unwrap().right = Some(Box::new(TreeNode::new(6)));
        assert!(!is_valid_bst(Some(root)));
    }
}
