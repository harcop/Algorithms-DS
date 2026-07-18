/// LeetCode #2471 - Minimum Number of Operations to Sort a Binary Tree by Level
use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    pub fn new(val: i32) -> Self {
        Self {
            val,
            left: None,
            right: None,
        }
    }
}

fn minimum_operations(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    let Some(root) = root else {
        return 0;
    };

    let mut answer = 0;
    let mut queue = VecDeque::from([root]);

    while !queue.is_empty() {
        let size = queue.len();
        let mut values = Vec::with_capacity(size);

        for _ in 0..size {
            let node = queue.pop_front().unwrap();
            let node = node.borrow();
            values.push(node.val);
            if let Some(left) = node.left.clone() {
                queue.push_back(left);
            }
            if let Some(right) = node.right.clone() {
                queue.push_back(right);
            }
        }

        let mut ids: Vec<usize> = (0..values.len()).collect();
        ids.sort_unstable_by_key(|&index| values[index]);

        for i in 0..ids.len() {
            while ids[i] != i {
                let target = ids[i];
                ids.swap(i, target);
                answer += 1;
            }
        }
    }

    answer
}

fn main() {
    let root = Some(Rc::new(RefCell::new(TreeNode::new(1))));
    println!("{}", minimum_operations(root));
}

#[cfg(test)]
mod tests {
    use super::{minimum_operations, TreeNode};
    use std::cell::RefCell;
    use std::rc::Rc;

    fn node(value: i32) -> Rc<RefCell<TreeNode>> {
        Rc::new(RefCell::new(TreeNode::new(value)))
    }

    #[test]
    fn example_one() {
        let root = node(1);
        let left = node(4);
        let right = node(3);
        left.borrow_mut().left = Some(node(7));
        left.borrow_mut().right = Some(node(6));
        right.borrow_mut().left = Some(node(8));
        right.borrow_mut().right = Some(node(5));
        right.borrow().left.as_ref().unwrap().borrow_mut().left = Some(node(9));
        right.borrow().right.as_ref().unwrap().borrow_mut().left = Some(node(10));
        root.borrow_mut().left = Some(left);
        root.borrow_mut().right = Some(right);

        assert_eq!(minimum_operations(Some(root)), 3);
    }

    #[test]
    fn example_two() {
        let root = node(1);
        let left = node(3);
        let right = node(2);
        left.borrow_mut().left = Some(node(7));
        left.borrow_mut().right = Some(node(6));
        right.borrow_mut().left = Some(node(5));
        right.borrow_mut().right = Some(node(4));
        root.borrow_mut().left = Some(left);
        root.borrow_mut().right = Some(right);

        assert_eq!(minimum_operations(Some(root)), 3);
    }

    #[test]
    fn already_sorted() {
        let root = node(1);
        root.borrow_mut().left = Some(node(2));
        root.borrow_mut().right = Some(node(3));
        assert_eq!(minimum_operations(Some(root)), 0);
    }
}
