/// LeetCode #971 - Flip Binary Tree To Match Preorder Traversal
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

fn flip_match_voyage(root: Option<Rc<RefCell<TreeNode>>>, voyage: Vec<i32>) -> Vec<i32> {
    let mut flipped = Vec::new();
    let mut idx = 0usize;
    fn dfs(
        node: &Option<Rc<RefCell<TreeNode>>>,
        voyage: &[i32],
        idx: &mut usize,
        flipped: &mut Vec<i32>,
    ) -> bool {
        if node.is_none() {
            return true;
        }
        let n = node.as_ref().unwrap();
        let val = n.borrow().val;
        if val != voyage[*idx] {
            return false;
        }
        *idx += 1;
        let left = n.borrow().left.clone();
        let right = n.borrow().right.clone();
        if left.is_none() || right.is_none() {
            return dfs(&left, voyage, idx, flipped) && dfs(&right, voyage, idx, flipped);
        }
        if left.as_ref().unwrap().borrow().val != voyage[*idx] {
            flipped.push(val);
            return dfs(&right, voyage, idx, flipped) && dfs(&left, voyage, idx, flipped);
        }
        dfs(&left, voyage, idx, flipped) && dfs(&right, voyage, idx, flipped)
    }
    if dfs(&root, &voyage, &mut idx, &mut flipped) && idx == voyage.len() {
        flipped
    } else {
        vec![-1]
    }
}

fn main() {
    let root = Rc::new(RefCell::new(TreeNode::new(1)));
    root.borrow_mut().left = Some(Rc::new(RefCell::new(TreeNode::new(2))));
    println!("{:?}", flip_match_voyage(Some(root), vec![1, 2, 3]));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_one() {
        let root = Rc::new(RefCell::new(TreeNode::new(1)));
        root.borrow_mut().left = Some(Rc::new(RefCell::new(TreeNode::new(2))));
        root.borrow_mut().right = Some(Rc::new(RefCell::new(TreeNode::new(3))));
        assert_eq!(
            flip_match_voyage(Some(root), vec![1, 3, 2]),
            vec![1]
        );
    }

    #[test]
    fn example_two() {
        let root = Rc::new(RefCell::new(TreeNode::new(1)));
        root.borrow_mut().left = Some(Rc::new(RefCell::new(TreeNode::new(2))));
        assert_eq!(
            flip_match_voyage(Some(root), vec![1, 2, 3]),
            vec![-1]
        );
    }
}
