/// LeetCode #1008 - Construct Binary Search Tree from Preorder Traversal
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

fn bst_from_preorder(preorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
    let mut idx = 0usize;
    fn build(preorder: &[i32], bound: i32, idx: &mut usize) -> Option<Rc<RefCell<TreeNode>>> {
        if *idx >= preorder.len() || preorder[*idx] > bound {
            return None;
        }
        let val = preorder[*idx];
        *idx += 1;
        let node = Rc::new(RefCell::new(TreeNode::new(val)));
        node.borrow_mut().left = build(preorder, val, idx);
        node.borrow_mut().right = build(preorder, bound, idx);
        Some(node)
    }
    build(&preorder, i32::MAX, &mut idx)
}

fn main() {
    let _ = bst_from_preorder(vec![8, 5, 1, 7, 10, 12]);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn inorder(root: &Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        match root {
            None => vec![],
            Some(n) => {
                let b = n.borrow();
                let mut v = inorder(&b.left);
                v.push(b.val);
                v.extend(inorder(&b.right));
                v
            }
        }
    }

    #[test]
    fn example_one() {
        let root = bst_from_preorder(vec![8, 5, 1, 7, 10, 12]);
        assert_eq!(inorder(&root), vec![1, 5, 7, 8, 10, 12]);
    }
}
