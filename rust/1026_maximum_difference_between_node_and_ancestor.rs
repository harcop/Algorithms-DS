/// LeetCode #1026 - Maximum Difference Between Node and Ancestor
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, Clone)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    pub fn new(val: i32) -> Self {
        TreeNode { val, left: None, right: None }
    }
}

fn max_ancestor_diff(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    fn dfs(node: &Option<Rc<RefCell<TreeNode>>>, min_v: i32, max_v: i32) -> i32 {
        if node.is_none() {
            return 0;
        }
        let v = node.as_ref().unwrap().borrow().val;
        let min_v = min_v.min(v);
        let max_v = max_v.max(v);
        let best = (v - min_v).max(max_v - v);
        let n = node.as_ref().unwrap().borrow();
        best.max(dfs(&n.left, min_v, max_v)).max(dfs(&n.right, min_v, max_v))
    }
    dfs(&root, i32::MAX, i32::MIN)
}

fn main() {
    let root = Rc::new(RefCell::new(TreeNode::new(8)));
    root.borrow_mut().left = Some(Rc::new(RefCell::new(TreeNode::new(3))));
    root.borrow_mut().right = Some(Rc::new(RefCell::new(TreeNode::new(10))));
    println!("{}", max_ancestor_diff(Some(root)));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_one() {
        let root = Rc::new(RefCell::new(TreeNode::new(8)));
        root.borrow_mut().left = Some(Rc::new(RefCell::new(TreeNode::new(3))));
        root.borrow_mut().right = Some(Rc::new(RefCell::new(TreeNode::new(10))));
        root.borrow().right.as_ref().unwrap().borrow_mut().right =
            Some(Rc::new(RefCell::new(TreeNode::new(14))));
        root.borrow().left.as_ref().unwrap().borrow_mut().left =
            Some(Rc::new(RefCell::new(TreeNode::new(1))));
        root.borrow().left.as_ref().unwrap().borrow_mut().right =
            Some(Rc::new(RefCell::new(TreeNode::new(6))));
        assert_eq!(max_ancestor_diff(Some(root)), 7);
    }
}
