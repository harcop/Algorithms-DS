/// LeetCode #114 - Flatten Binary Tree to Linked List
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

fn flatten(root: &mut Option<Box<TreeNode>>) {
    let mut vals = Vec::new();
    fn preorder(node: &Option<Box<TreeNode>>, vals: &mut Vec<i32>) {
        if let Some(n) = node {
            vals.push(n.val);
            preorder(&n.left, vals);
            preorder(&n.right, vals);
        }
    }
    preorder(root, &mut vals);
    if vals.is_empty() {
        return;
    }

    let mut new_root = Box::new(TreeNode::new(vals[0]));
    let mut cur = &mut new_root;
    for &v in vals.iter().skip(1) {
        cur.right = Some(Box::new(TreeNode::new(v)));
        cur = cur.right.as_mut().unwrap();
    }
    *root = Some(new_root);
}

fn linked_right_walk(mut root: Option<Box<TreeNode>>) -> Vec<i32> {
    let mut out = Vec::new();
    while let Some(node) = root {
        out.push(node.val);
        root = node.right;
    }
    out
}

fn main() {
    let mut root = Some(Box::new(TreeNode::new(1)));
    flatten(&mut root);
    println!("{:?}", linked_right_walk(root));
}

#[cfg(test)]
mod tests {
    use super::{flatten, linked_right_walk, TreeNode};

    #[test]
    fn example_one() {
        let mut root = Some(Box::new(TreeNode::new(1)));
        root.as_mut().unwrap().left = Some(Box::new(TreeNode::new(2)));
        root.as_mut().unwrap().right = Some(Box::new(TreeNode::new(5)));
        root.as_mut().unwrap().left.as_mut().unwrap().left = Some(Box::new(TreeNode::new(3)));
        root.as_mut().unwrap().left.as_mut().unwrap().right = Some(Box::new(TreeNode::new(4)));
        root.as_mut().unwrap().right.as_mut().unwrap().right = Some(Box::new(TreeNode::new(6)));
        flatten(&mut root);
        assert_eq!(linked_right_walk(root), vec![1, 2, 3, 4, 5, 6]);
    }

    #[test]
    fn example_two() {
        flatten(&mut None);
    }
}
