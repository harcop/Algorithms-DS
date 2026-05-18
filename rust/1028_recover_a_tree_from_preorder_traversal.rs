/// LeetCode #1028 - Recover a Tree From Preorder Traversal
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

fn recover_from_preorder(traversal: String) -> Option<Rc<RefCell<TreeNode>>> {
    let bytes = traversal.as_bytes();
    let mut idx = 0usize;
    fn build(bytes: &[u8], depth: usize, idx: &mut usize) -> Option<Rc<RefCell<TreeNode>>> {
        let mut val = 0i32;
        while *idx < bytes.len() && bytes[*idx].is_ascii_digit() {
            val = val * 10 + (bytes[*idx] - b'0') as i32;
            *idx += 1;
        }
        let node = Rc::new(RefCell::new(TreeNode::new(val)));
        if *idx < bytes.len() && bytes[*idx] == b'-' {
            while *idx < bytes.len() && bytes[*idx] == b'-' {
                *idx += 1;
            }
            node.borrow_mut().left = build(bytes, depth + 1, idx);
        }
        if *idx < bytes.len() && bytes[*idx] == b'-' {
            while *idx < bytes.len() && bytes[*idx] == b'-' {
                *idx += 1;
            }
            node.borrow_mut().right = build(bytes, depth + 1, idx);
        }
        Some(node)
    }
    build(&bytes, 0, &mut idx)
}

fn main() {
    let _ = recover_from_preorder("1-2--3---4-5--6-7--8".into());
}

#[cfg(test)]
mod tests {
    use super::*;

    fn preorder(root: &Option<Rc<RefCell<TreeNode>>>) -> String {
        match root {
            None => String::new(),
            Some(n) => {
                let b = n.borrow();
                format!("{}{}{}", b.val, pre(&b.left, 1), pre(&b.right, 1))
            }
        }
    }
    fn pre(root: &Option<Rc<RefCell<TreeNode>>>, d: usize) -> String {
        match root {
            None => String::new(),
            Some(n) => {
                let b = n.borrow();
                format!("{}{}{}", "-".repeat(d), b.val, pre(&b.left, d + 1) + &pre(&b.right, d + 1))
            }
        }
    }

    #[test]
    fn example_one() {
        let root = recover_from_preorder("1-2--3---4-5--6-7--8".into());
        assert_eq!(preorder(&root), "1-2--3---4-5--6-7--8");
    }
}
