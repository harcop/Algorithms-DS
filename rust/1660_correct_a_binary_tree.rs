/// LeetCode #1660 - Correct A Binary Tree
use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Box<TreeNode>>,
    pub right: Option<Box<TreeNode>>,
}

fn correct_tree(root: Option<Box<TreeNode>>) -> Option<Box<TreeNode>> {
    let mut seen = HashMap::new();
    let mut dup = None;
    fn mark(node: &Option<Box<TreeNode>>, seen: &mut HashMap<i32, i32>, dup: &mut Option<i32>) {
        let Some(n) = node else { return; };
        if seen.insert(n.val, 1).is_some() { *dup = Some(n.val); }
        mark(&n.left, seen, dup);
        mark(&n.right, seen, dup);
    }
    mark(&root, &mut seen, &mut dup);
    let dup = dup?;
    fn fix(node: &mut Option<Box<TreeNode>>, dup: i32) -> bool {
        let Some(n) = node else { return false; };
        if fix(&mut n.left, dup) || fix(&mut n.right, dup) { return true; }
        if n.val == dup {
            if let Some(l) = n.left.take() {
                if l.val == dup { n.left = None; return true; }
            }
            if let Some(r) = n.right.take() {
                if r.val == dup { n.right = None; return true; }
            }
        }
        false
    }
    let mut root = root;
    fix(&mut root, dup);
    root
}
fn main() { println!("{:?}", correct_tree(None)); }
#[cfg(test)]
mod tests {
    use super::{correct_tree, TreeNode};
    #[test]
    fn example_one() {
        let root = Some(Box::new(TreeNode {
            val: 5,
            left: Some(Box::new(TreeNode { val: 2, left: None, right: None })),
            right: Some(Box::new(TreeNode {
                val: 2,
                left: Some(Box::new(TreeNode { val: 1, left: None, right: None })),
                right: None,
            })),
        }));
        let r = correct_tree(root);
        assert!(r.is_some());
    }
}