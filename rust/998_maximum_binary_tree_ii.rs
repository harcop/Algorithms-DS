/// LeetCode #998 - Maximum Binary Tree II
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

fn insert_into_max_tree(mut root: Option<Rc<RefCell<TreeNode>>>, val: i32) -> Option<Rc<RefCell<TreeNode>>> {
    let node = Rc::new(RefCell::new(TreeNode::new(val)));
    if root.is_none() {
        return Some(node);
    }
    let mut cur = root.clone();
    let mut parent: Option<Rc<RefCell<TreeNode>>> = None;
    let mut is_left = false;
    while let Some(r) = cur {
        let v = r.borrow().val;
        if val > v {
            break;
        }
        parent = Some(r.clone());
        if r.borrow().right.is_some() {
            cur = r.borrow().right.clone();
            is_left = false;
        } else {
            node.borrow_mut().left = Some(r.clone());
            if let Some(p) = &parent {
                if is_left {
                    p.borrow_mut().left = Some(node.clone());
                } else {
                    p.borrow_mut().right = Some(node.clone());
                }
            }
            return root;
        }
    }
    node.borrow_mut().left = root;
    Some(node)
}

fn main() {
    let root = Rc::new(RefCell::new(TreeNode::new(4)));
    root.borrow_mut().left = Some(Rc::new(RefCell::new(TreeNode::new(1))));
    let _ = insert_into_max_tree(Some(root), 5);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn vals(mut root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut out = Vec::new();
        while let Some(r) = root {
            out.push(r.borrow().val);
            root = r.borrow().right.clone().or_else(|| r.borrow().left.clone());
        }
        out
    }

    #[test]
    fn example_one() {
        let root = Rc::new(RefCell::new(TreeNode::new(4)));
        root.borrow_mut().left = Some(Rc::new(RefCell::new(TreeNode::new(1))));
        let root = insert_into_max_tree(Some(root), 5);
        assert_eq!(vals(root), vec![5, 4, 1]);
    }
}
