/// LeetCode #988 - Smallest String Starting From Leaf
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

fn smallest_from_leaf(root: Option<Rc<RefCell<TreeNode>>>) -> String {
    let mut best: Option<String> = None;
    fn dfs(node: &Option<Rc<RefCell<TreeNode>>>, path: &mut Vec<u8>, best: &mut Option<String>) {
        if node.is_none() { return; }
        let n = node.as_ref().unwrap().borrow();
        path.push((n.val as u8) + b'a');
        if n.left.is_none() && n.right.is_none() {
            let s: String = path.iter().rev().map(|&c| c as char).collect();
            if best.as_ref().map(|b| s < *b).unwrap_or(true) {
                *best = Some(s);
            }
        }
        dfs(&n.left, path, best);
        dfs(&n.right, path, best);
        path.pop();
    }
    let mut path = Vec::new();
    dfs(&root, &mut path, &mut best);
    best.unwrap_or_default()
}

fn main() {
    let root = Rc::new(RefCell::new(TreeNode::new(0)));
    root.borrow_mut().left = Some(Rc::new(RefCell::new(TreeNode::new(1))));
    root.borrow_mut().right = Some(Rc::new(RefCell::new(TreeNode::new(2))));
    println!("{}", smallest_from_leaf(Some(root)));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_one() {
        let root = Rc::new(RefCell::new(TreeNode::new(0)));
        root.borrow_mut().left = Some(Rc::new(RefCell::new(TreeNode::new(1))));
        root.borrow_mut().right = Some(Rc::new(RefCell::new(TreeNode::new(2))));
        root.borrow().left.as_ref().unwrap().borrow_mut().left =
            Some(Rc::new(RefCell::new(TreeNode::new(3))));
        root.borrow().left.as_ref().unwrap().borrow_mut().right =
            Some(Rc::new(RefCell::new(TreeNode::new(4))));
        root.borrow().right.as_ref().unwrap().borrow_mut().left =
            Some(Rc::new(RefCell::new(TreeNode::new(3))));
        root.borrow().right.as_ref().unwrap().borrow_mut().right =
            Some(Rc::new(RefCell::new(TreeNode::new(4))));
        assert_eq!(smallest_from_leaf(Some(root)), "dba");
    }
}
