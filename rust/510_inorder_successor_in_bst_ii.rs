/// LeetCode #510 - Inorder Successor in BST II
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

fn inorder_successor(root: Option<&Box<TreeNode>>, p: i32) -> Option<i32> {
    let mut succ = None;
    let mut cur = root;
    while let Some(n) = cur {
        if n.val > p {
            succ = Some(n.val);
            cur = n.left.as_ref();
        } else {
            cur = n.right.as_ref();
        }
    }
    succ
}

fn main() {
    let mut root = Box::new(TreeNode::new(2));
    root.left = Some(Box::new(TreeNode::new(1)));
    root.right = Some(Box::new(TreeNode::new(3)));
    println!("{:?}", inorder_successor(Some(&root), 1));
}

#[cfg(test)]
mod tests {
    use super::{inorder_successor, TreeNode};

    #[test]
    fn example_one() {
        let mut root = Box::new(TreeNode::new(2));
        root.left = Some(Box::new(TreeNode::new(1)));
        root.right = Some(Box::new(TreeNode::new(3)));
        assert_eq!(inorder_successor(Some(&root), 1), Some(2));
    }

    #[test]
    fn example_two() {
        let mut root = Box::new(TreeNode::new(5));
        let mut l = Box::new(TreeNode::new(3));
        l.left = Some(Box::new(TreeNode::new(2)));
        l.right = Some(Box::new(TreeNode::new(4)));
        l.left.as_mut().unwrap().left = Some(Box::new(TreeNode::new(1)));
        root.left = Some(l);
        root.right = Some(Box::new(TreeNode::new(6)));
        assert_eq!(inorder_successor(Some(&root), 6), None);
    }
}
