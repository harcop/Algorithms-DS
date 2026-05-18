/// LeetCode #979 - Distribute Coins in Binary Tree
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

fn distribute_coins(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    let mut moves = 0i32;
    fn dfs(node: &Option<Rc<RefCell<TreeNode>>>, moves: &mut i32) -> i32 {
        if node.is_none() {
            return 0;
        }
        let n = node.as_ref().unwrap();
        let left = dfs(&n.borrow().left, moves);
        let right = dfs(&n.borrow().right, moves);
        let excess = n.borrow().val - 1 + left + right;
        *moves += excess.abs();
        excess
    }
    dfs(&root, &mut moves);
    moves
}

fn main() {
    let root = Rc::new(RefCell::new(TreeNode::new(3)));
    root.borrow_mut().left = Some(Rc::new(RefCell::new(TreeNode::new(0))));
    root.borrow_mut().right = Some(Rc::new(RefCell::new(TreeNode::new(0))));
    println!("{}", distribute_coins(Some(root)));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_one() {
        let root = Rc::new(RefCell::new(TreeNode::new(3)));
        root.borrow_mut().left = Some(Rc::new(RefCell::new(TreeNode::new(0))));
        root.borrow_mut().right = Some(Rc::new(RefCell::new(TreeNode::new(0))));
        assert_eq!(distribute_coins(Some(root)), 2);
    }

    #[test]
    fn example_two() {
        let root = Rc::new(RefCell::new(TreeNode::new(0)));
        root.borrow_mut().left = Some(Rc::new(RefCell::new(TreeNode::new(3))));
        root.borrow_mut().right = Some(Rc::new(RefCell::new(TreeNode::new(0))));
        assert_eq!(distribute_coins(Some(root)), 3);
    }
}
