/// LeetCode #1325 - Delete Leaves with a Given Value
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

fn remove_leaf_nodes(root: Option<Rc<RefCell<TreeNode>>>, target: i32) -> Option<Rc<RefCell<TreeNode>>> {
    fn dfs(node: Option<Rc<RefCell<TreeNode>>>, target: i32) -> Option<Rc<RefCell<TreeNode>>> {
        let node = node?;
        let mut n = node.borrow_mut();
        n.left = dfs(n.left.take(), target);
        n.right = dfs(n.right.take(), target);
        let is_leaf = n.left.is_none() && n.right.is_none();
        drop(n);
        if is_leaf && node.borrow().val == target {
            None
        } else {
            Some(node)
        }
    }
    dfs(root, target)
}

fn main() {
    println!("{:?}", remove_leaf_nodes(None, 1).is_none());
}

#[cfg(test)]
mod tests {
    use super::{remove_leaf_nodes, TreeNode};
    use std::cell::RefCell;
    use std::rc::Rc;

    #[test]
    fn example_one() {
        let root = Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: Some(Rc::new(RefCell::new(TreeNode { val: 2, left: None, right: None }))),
                right: None,
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 3,
                left: None,
                right: Some(Rc::new(RefCell::new(TreeNode { val: 2, left: None, right: None }))),
            }))),
        }));
        let out = remove_leaf_nodes(Some(root), 2);
        assert!(out.is_some());
    }
}
