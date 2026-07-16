/// LeetCode #2415 - Reverse Odd Levels of Binary Tree
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

fn reverse_odd_levels(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
    fn dfs(
        left: Option<Rc<RefCell<TreeNode>>>,
        right: Option<Rc<RefCell<TreeNode>>>,
        level: i32,
    ) {
        if let (Some(left_node), Some(right_node)) = (left, right) {
            if level % 2 == 1 {
                let left_val = left_node.borrow().val;
                let right_val = right_node.borrow().val;
                left_node.borrow_mut().val = right_val;
                right_node.borrow_mut().val = left_val;
            }

            let (ll, lr, rl, rr) = {
                let left_ref = left_node.borrow();
                let right_ref = right_node.borrow();
                (
                    left_ref.left.clone(),
                    left_ref.right.clone(),
                    right_ref.left.clone(),
                    right_ref.right.clone(),
                )
            };
            dfs(ll, rr, level + 1);
            dfs(lr, rl, level + 1);
        }
    }

    if let Some(node) = root.clone() {
        let (left, right) = {
            let node_ref = node.borrow();
            (node_ref.left.clone(), node_ref.right.clone())
        };
        dfs(left, right, 1);
    }
    root
}

fn main() {
    let root = Some(Rc::new(RefCell::new(TreeNode::new(2))));
    println!("{:?}", reverse_odd_levels(root).is_some());
}

#[cfg(test)]
mod tests {
    use super::{reverse_odd_levels, TreeNode};
    use std::cell::RefCell;
    use std::rc::Rc;

    fn node(val: i32) -> Option<Rc<RefCell<TreeNode>>> {
        Some(Rc::new(RefCell::new(TreeNode::new(val))))
    }

    #[test]
    fn example_one() {
        let root = node(2);
        {
            let root_ref = root.as_ref().unwrap();
            root_ref.borrow_mut().left = node(3);
            root_ref.borrow_mut().right = node(5);

            let left = root_ref.borrow().left.clone().unwrap();
            left.borrow_mut().left = node(8);
            left.borrow_mut().right = node(13);

            let right = root_ref.borrow().right.clone().unwrap();
            right.borrow_mut().left = node(21);
            right.borrow_mut().right = node(34);
        }

        let result = reverse_odd_levels(root).unwrap();
        let left_val = result.borrow().left.as_ref().unwrap().borrow().val;
        let right_val = result.borrow().right.as_ref().unwrap().borrow().val;
        assert_eq!((left_val, right_val), (5, 3));
    }

    #[test]
    fn single_node() {
        let root = node(7);
        let result = reverse_odd_levels(root).unwrap();
        assert_eq!(result.borrow().val, 7);
    }
}
