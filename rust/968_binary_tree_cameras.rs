/// LeetCode #968 - Binary Tree Cameras
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
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

fn min_camera_cover(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    fn dfs(node: &Option<Rc<RefCell<TreeNode>>>) -> (i32, i32) {
        if node.is_none() {
            return (0, 1);
        }
        let n = node.as_ref().unwrap();
        let (lc, ls) = dfs(&n.borrow().left);
        let (rc, rs) = dfs(&n.borrow().right);
        if ls == 0 || rs == 0 {
            return (lc + rc + 1, 2);
        }
        if ls == 2 || rs == 2 {
            return (lc + rc, 1);
        }
        (lc + rc, 0)
    }
    let (cams, state) = dfs(&root);
    if state == 0 {
        cams + 1
    } else {
        cams
    }
}

fn main() {
    let root = Rc::new(RefCell::new(TreeNode::new(0)));
    root.borrow_mut().left = Some(Rc::new(RefCell::new(TreeNode::new(0))));
    root.borrow_mut().right = Some(Rc::new(RefCell::new(TreeNode::new(0))));
    println!("{}", min_camera_cover(Some(root)));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_one() {
        let root = Rc::new(RefCell::new(TreeNode::new(0)));
        root.borrow_mut().left = Some(Rc::new(RefCell::new(TreeNode::new(0))));
        root.borrow_mut().right = Some(Rc::new(RefCell::new(TreeNode::new(0))));
        assert_eq!(min_camera_cover(Some(root)), 1);
    }

    #[test]
    fn example_two() {
        // [0,0,null,0,null,0,0]
        let n6 = Rc::new(RefCell::new(TreeNode::new(0)));
        let n5 = Rc::new(RefCell::new(TreeNode::new(0)));
        let n4 = Rc::new(RefCell::new(TreeNode::new(0)));
        n4.borrow_mut().right = Some(n6.clone());
        n4.borrow_mut().left = Some(n5.clone());
        let n2 = Rc::new(RefCell::new(TreeNode::new(0)));
        n2.borrow_mut().left = Some(n4);
        let root = Rc::new(RefCell::new(TreeNode::new(0)));
        root.borrow_mut().left = Some(n2);
        assert_eq!(min_camera_cover(Some(root)), 2);
    }
}
