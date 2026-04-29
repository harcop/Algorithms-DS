/// LeetCode #99 - Recover Binary Search Tree
///
/// Two nodes were swapped by mistake. Restoring BST property:
/// in-order traversal values must be sorted; replacing them with the sorted
/// multiset fixes exactly two swapped values (problem constraint).
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

fn recover_tree(root: &mut Option<Box<TreeNode>>) {
    let mut ptrs: Vec<*mut i32> = Vec::new();

    fn collect(node: &mut Option<Box<TreeNode>>, ptrs: &mut Vec<*mut i32>) {
        if let Some(n) = node.as_mut() {
            collect(&mut n.left, ptrs);
            ptrs.push(&mut n.val as *mut i32);
            collect(&mut n.right, ptrs);
        }
    }

    collect(root, &mut ptrs);
    let mut values: Vec<i32> = ptrs.iter().map(|p| unsafe { **p }).collect();
    values.sort_unstable();

    for (p, &v) in ptrs.iter().zip(values.iter()) {
        unsafe {
            **p = v;
        }
    }
}

fn inorder_vals(root: &Option<Box<TreeNode>>) -> Vec<i32> {
    let mut out = Vec::new();
    fn walk(node: &Option<Box<TreeNode>>, out: &mut Vec<i32>) {
        if let Some(n) = node {
            walk(&n.left, out);
            out.push(n.val);
            walk(&n.right, out);
        }
    }
    walk(root, &mut out);
    out
}

fn main() {
    let mut root = Some(Box::new(TreeNode::new(1)));
    root.as_mut().unwrap().left = Some(Box::new(TreeNode::new(3)));
    root.as_mut().unwrap().right = Some(Box::new(TreeNode::new(2)));
    recover_tree(&mut root);
    println!("{:?}", inorder_vals(&root));
}

#[cfg(test)]
mod tests {
    use super::{inorder_vals, recover_tree, TreeNode};

    #[test]
    fn example_one() {
        let mut root = Some(Box::new(TreeNode::new(1)));
        root.as_mut().unwrap().left = Some(Box::new(TreeNode::new(3)));
        root.as_mut().unwrap().right = Some(Box::new(TreeNode::new(2)));
        recover_tree(&mut root);
        assert_eq!(inorder_vals(&root), vec![1, 2, 3]);
    }

    #[test]
    fn example_two() {
        let mut root = Some(Box::new(TreeNode::new(3)));
        root.as_mut().unwrap().left = Some(Box::new(TreeNode::new(1)));
        root.as_mut().unwrap().right = Some(Box::new(TreeNode::new(4)));
        root.as_mut().unwrap().right.as_mut().unwrap().left = Some(Box::new(TreeNode::new(2)));
        recover_tree(&mut root);
        assert_eq!(inorder_vals(&root), vec![1, 2, 3, 4]);
    }
}
