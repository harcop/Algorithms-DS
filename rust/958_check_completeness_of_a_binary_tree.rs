/// LeetCode #958 - Check Completeness of a Binary Tree
use std::collections::VecDeque;
use std::rc::Rc;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<TreeNode>>,
    pub right: Option<Rc<TreeNode>>,
}

fn is_complete_tree(root: Option<Rc<TreeNode>>) -> bool {
    let Some(root) = root else {
        return true;
    };
    let mut q = VecDeque::new();
    q.push_back(root);
    let mut end = false;
    while let Some(node) = q.pop_front() {
        if end {
            if node.left.is_some() || node.right.is_some() {
                return false;
            }
            continue;
        }
        match (&node.left, &node.right) {
            (None, None) => end = true,
            (Some(l), None) => {
                q.push_back(l.clone());
                end = true;
            }
            (Some(l), Some(r)) => {
                q.push_back(l.clone());
                q.push_back(r.clone());
            }
            (None, Some(_)) => return false,
        }
    }
    true
}

fn main() {
    let root = Rc::new(TreeNode {
        val: 1,
        left: Some(Rc::new(TreeNode { val: 2, left: None, right: None })),
        right: Some(Rc::new(TreeNode { val: 3, left: None, right: None })),
    });
    println!("{}", is_complete_tree(Some(root)));
}

#[cfg(test)]
mod tests {
    use super::{is_complete_tree, TreeNode};
    use std::rc::Rc;

    #[test]
    fn example_one() {
        let root = Rc::new(TreeNode {
            val: 1,
            left: Some(Rc::new(TreeNode {
                val: 2,
                left: Some(Rc::new(TreeNode { val: 4, left: None, right: None })),
                right: Some(Rc::new(TreeNode { val: 5, left: None, right: None })),
            })),
            right: Some(Rc::new(TreeNode {
                val: 3,
                left: Some(Rc::new(TreeNode { val: 6, left: None, right: None })),
                right: None,
            })),
        });
        assert!(is_complete_tree(Some(root)));
    }

    #[test]
    fn example_two() {
        let root = Rc::new(TreeNode {
            val: 1,
            left: Some(Rc::new(TreeNode {
                val: 2,
                left: Some(Rc::new(TreeNode { val: 4, left: None, right: None })),
                right: Some(Rc::new(TreeNode { val: 5, left: None, right: None })),
            })),
            right: Some(Rc::new(TreeNode {
                val: 3,
                left: None,
                right: Some(Rc::new(TreeNode { val: 7, left: None, right: None })),
            })),
        });
        assert!(!is_complete_tree(Some(root)));
    }
}
