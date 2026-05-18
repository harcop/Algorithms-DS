/// LeetCode #897 - Increasing Order Search Tree
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

fn increasing_bst(root: Option<Box<TreeNode>>) -> Option<Box<TreeNode>> {
    let mut vals = Vec::new();
    fn inorder(n: &Option<Box<TreeNode>>, vals: &mut Vec<i32>) {
        if let Some(node) = n {
            inorder(&node.left, vals);
            vals.push(node.val);
            inorder(&node.right, vals);
        }
    }
    inorder(&root, &mut vals);
    if vals.is_empty() {
        return None;
    }
    let mut h = Box::new(TreeNode::new(vals[0]));
    let mut cur = h.as_mut();
    for &v in vals.iter().skip(1) {
        cur.right = Some(Box::new(TreeNode::new(v)));
        cur = cur.right.as_mut().unwrap();
    }
    Some(h)
}

fn main() {
    let mut r = Box::new(TreeNode::new(5));
    r.left = Some(Box::new(TreeNode::new(3)));
    r.right = Some(Box::new(TreeNode::new(6)));
    println!("{:?}", increasing_bst(Some(r)));
}

#[cfg(test)]
mod tests {
    use super::{increasing_bst, TreeNode};

    #[test]
    fn example_one() {
        let mut r = Box::new(TreeNode::new(5));
        r.left = Some(Box::new(TreeNode::new(1)));
        r.right = Some(Box::new(TreeNode::new(7)));
        let out = increasing_bst(Some(r));
        assert!(out.is_some());
    }
}
