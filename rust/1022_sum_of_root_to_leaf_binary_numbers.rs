/// LeetCode #1022 - Sum of Root To Leaf Binary Numbers
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

fn sum_root_to_leaf(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    fn dfs(node: &Option<Rc<RefCell<TreeNode>>>, cur: i32) -> i32 {
        if node.is_none() {
            return 0;
        }
        let n = node.as_ref().unwrap().borrow();
        let cur = cur * 2 + n.val;
        if n.left.is_none() && n.right.is_none() {
            return cur;
        }
        dfs(&n.left, cur) + dfs(&n.right, cur)
    }
    dfs(&root, 0)
}

fn main() {
    let root = Rc::new(RefCell::new(TreeNode::new(1)));
    root.borrow_mut().left = Some(Rc::new(RefCell::new(TreeNode::new(0))));
    root.borrow_mut().right = Some(Rc::new(RefCell::new(TreeNode::new(1))));
    println!("{}", sum_root_to_leaf(Some(root)));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_one() {
        let root = Rc::new(RefCell::new(TreeNode::new(1)));
        root.borrow_mut().left = Some(Rc::new(RefCell::new(TreeNode::new(0))));
        root.borrow_mut().right = Some(Rc::new(RefCell::new(TreeNode::new(1))));
        root.borrow().left.as_ref().unwrap().borrow_mut().left =
            Some(Rc::new(RefCell::new(TreeNode::new(0))));
        root.borrow().left.as_ref().unwrap().borrow_mut().right =
            Some(Rc::new(RefCell::new(TreeNode::new(1))));
        root.borrow().right.as_ref().unwrap().borrow_mut().left =
            Some(Rc::new(RefCell::new(TreeNode::new(0))));
        assert_eq!(sum_root_to_leaf(Some(root)), 22);
    }
}
